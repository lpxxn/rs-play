use std::time::Duration;

mod lib;

use lib::TimerFuture;

fn main() {
    let (executor, spawner) = lib::new_executor_and_spawner();

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