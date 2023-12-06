use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use std::task::Poll;
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

        TimerFuture{
            shared_state
        }
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
