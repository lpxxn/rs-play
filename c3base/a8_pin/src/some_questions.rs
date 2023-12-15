use futures::future::{BoxFuture, FutureExt};


/*
// 因此 recursive 函数
async fn recursive() {
    recursive().await;
    recursive().await;
}

// 会生成类似以下的类型
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
就算是使用 Box，这里也大有讲究。如果我们试图使用 Box::pin 这种方式去包裹是不行的，因为编译器自身的限制限制了我们(刚夸过它。。。)。
为了解决这种问题，我们只能将 recursive 转变成一个正常的函数，该函数返回一个使用 Box 包裹的 async 语句块：
 */
fn recursive() -> BoxFuture<'static, ()> {
    async move {
        println!("recursive");
        recursive().await;
    }.boxed()
}

//在特征中使用 async
use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
    async fn async_fn(&self);
}

struct Modal;

#[async_trait]
impl AsyncTrait for Modal {
    async fn async_fn(&self) {
        println!("async fn");
    }
}





