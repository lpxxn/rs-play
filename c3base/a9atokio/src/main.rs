async fn say_to_world() -> String {
    "world".to_string()
}

#[tokio::main]
async fn main() {
    let op = say_to_world();
    println!("hello: {}", "aaaaa");
    let op = op.await;
    println!("op: {:?}", op);
}

/* 展开成这样
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
 */