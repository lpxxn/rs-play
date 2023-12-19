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
