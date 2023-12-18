#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("in tokio spawn")
    });
    // The provided future will start running in the background immediately when spawn is called,
    // even if you don’t await the returned JoinHandle.
    let handle = tokio::spawn(async {
        println!("in tokio spawn 2")
    });
    let handle = tokio::spawn(async move{
        1111
    });

    let out = handle.await.unwrap();
    println!("GOT: {}", out);
}