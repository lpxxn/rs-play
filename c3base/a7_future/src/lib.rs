use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, mpsc::{Receiver, SyncSender, sync_channel}},
    thread,
    time::Duration, mem::ManuallyDrop, task::Context,
};
use std::task::Poll;

use futures::{future::BoxFuture, FutureExt, task::{ArcWake, waker_ref}};
/*
新建线程在睡眠结束后会需要将状态同步给定时器 Future ，由于是多线程环境，我们需要使用 Arc<Mutex<T>> 来作为一个共享状态，用于在新线程和 Future 定时器间共享。
 */

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线徎间共享状态
struct SharedState {
    /// 定时（睡眠）是否结束
    completed: bool,
    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<std::task::Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置`waker`， 这样新线程在睡眠（计时）结束后可以唤醒当前的任务
            // 然后Future的执行器会再次调用 `poll`方法
            // 下面的`clone` 每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更合理。
            // 选择每次都`clone`的原因是：`TimerFuture`可以在执行器的不同任务间移动，如果只clone一次
            // 那么获取到的`warker`可能已经被篡改并指向其他任务，最张执器会错误的唤醒其他任务。
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// 创建一个 API 用于构建定时器和启动计时线程:
impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 将completed设置为true 唤醒在此定时器上阻塞的任务
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture {
            shared_state
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (executor, spawner) = new_executor_and_spawner();

        // 生成一个任务
        spawner.spawn(async {
            println!("howdy!");
            // 创建Future,并等待完成
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done");
        });
        // drop 掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
        drop(spawner);
        // 运行执行器直到任务队列为空
        // 任务运行后，会先打印howdy!，停2秒后，再打印done
        executor.run();
    }
}

/// 任务执行器，负责从通道中接收任务然后执行
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将他发送到任务中
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}


/// 一个Future，它可以高度自己（将自己放入任务通道中），然后等待执行器去`Poll`
struct Task {
    /// 进行中的Future,在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但由于
    /// 我们需要使用`Mutex`来满足不太聪明的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自身放回到任务通道中，等待执行器pool
    task_sender: SyncSender<Arc<Task>>,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数（任务队列的最大长度）
    // 当任务的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

// 添加一个方法，用于生成 Future, 然后将他放入任务通道中
impl Spawner {
    pub fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        // 
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// 在执行器 pool 一个Future之前，首先需要调用  wake 方法进行唤醒，然后再由wake 负责调度该任务并将其放入任务通道中。创建waker的最简单方式就是实现ArcWake特征
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`,这样`wake`后，任务就能被执行 poll
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

// 当任务实现了 ArcWake 特征后，它就变成了 Waker ，在调用 wake() 对其唤醒后会将任务复制一份所有权( Arc )，
// 然后将其发送到任务通道中。最后我们的执行器将从通道中获取任务，然后进行 poll 执行
impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future, 苦它还有没有完成（仍然是Some，不是None)，则对它进行一次poll并尝试完成他
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 LocalWaker
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // BoxFuture<T> 是Pin<Box<dyn Future<Output = T> + Send + 'static>> 的类型别名
                // 通过调用 as_mut 方法，可以将上面的类型转换成 Pin<&mut dyn Future + Send + 'static>
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将他放回到任务中，等待下次被 poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}