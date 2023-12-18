use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // 语句块的使用强制了 rc会在await 被调用之前就被
        // 释放，因此rc并不会影响.await的安全性
        // 注释掉{}可以试试
        {
            let rc = Rc::new("hello");
            println!("{rc}");
        }
        // `rc` 的作用范围已经失效，因此当任务让出所有权给当前线程时，它无需作为状态被保存起来
        yield_now().await;

        // 事实上，注释掉下面一行代码，依然会报错
        // 原因是：是否保存，不取决于 `rc` 是否被使用，而是取决于 `.await`在调用时是否仍然处于 `rc` 的作用域中
        // println!("{}", rc);

        // rc 作用域在这里结束
    });
}