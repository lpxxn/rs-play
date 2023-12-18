use std::collections::HashMap;
use std::sync::{Arc};
use tokio::net::{TcpListener, TcpStream};
// use std::sync::Mutex;
use tokio::sync::Mutex;
use mini_redis::{Command, Connection, Frame};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();
    println!("redis server running...");
    let db = Arc::new(Mutex::new(HashMap::new()));


    loop {
        // 第二个被忽略的项中包含有新连接的 IP和端口信息
        let (socket, add) = listener.accept().await.unwrap();
        println!("addr: {:?}", add);
        // 先将db clone一份
        let db = db.clone();
        // 为每一条连接都生成一个新的任务
        // socket 的所有权将被移到新的任务中，并在那里进行处理
        tokio::spawn(async {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{Get, Set};
    use std::collections::HashMap;

    // connection  对于redis的读取进行了抽象，读取的是一个一个数据帧frame(数据帧 = redis 命令+数据),而
    // 不是字节流
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT {:?}", frame);

        // 回复一个错误
        // let response = Frame::Error("unimplemented".to_string());

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // let mut db = db.lock().unwrap();
                let mut db = db.lock().await;
                // 值被存储为 Vec<u8> 的形式
                db.insert(cmd.key().to_string(), cmd.value().clone());// 这里改成了value.clone
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                // let db = db.lock().unwrap();
                let db = db.lock().await;
                if let Some(value) = db.get(cmd.key()) {
                    let v = value.clone();
                    Frame::Bulk(v)
                } else {
                    Frame::Null
                }
            }
            cmd => {
                // panic!("unimplemented {:?}", cmd)
                Frame::Simple("OK".to_string())
            }
        };
        // 将响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
}

/*
上面代码还有一点非常重要，那就是我们使用了 std::sync::Mutex 来保护 HashMap，而不是使用 tokio::sync::Mutex。

在使用 Tokio 编写异步代码时，一个常见的错误无条件地使用 tokio::sync::Mutex ，而真相是：Tokio 提供的异步锁只应该在跨多个 .await调用时使用，而且 Tokio 的 Mutex 实际上内部使用的也是 std::sync::Mutex。

多补充几句，在异步代码中，关于锁的使用有以下经验之谈：

锁如果在多个 .await 过程中持有，应该使用 Tokio 提供的锁，原因是 .await的过程中锁可能在线程间转移，若使用标准库的同步锁存在死锁的可能性，例如某个任务刚获取完锁，还没使用完就因为 .await 让出了当前线程的所有权，结果下个任务又去获取了锁，造成死锁
锁竞争不多的情况下，使用 std::sync::Mutex
锁竞争多，可以考虑使用三方库提供的性能更高的锁，例如 parking_lot::Mutex
 */