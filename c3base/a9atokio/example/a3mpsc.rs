use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    // 当所有的发送者都被 Drop 掉后(超出作用域或被 drop(...) 函数主动释放)，就不再会有任何消息发送给该通道，此时 recv 方法将返回 None，也意味着该通道已经被关闭。
    // 去掉注释，试试
    // let tx3 = tx.clone();
    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(msg) = rx.recv().await {
        println!("GOG: {msg}");
    }
    // tx3.send("3");
    println!("ending...")
}