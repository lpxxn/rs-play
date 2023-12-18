use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Command, Connection, Frame};

#[tokio::main]
async fn main() {
    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();
    loop {
        // 第二个被忽略的项中包含有新连接的 IP和端口信息
        let (socket, add) = listener.accept().await.unwrap();
        println!("addr: {:?}", add);
        // 为每一条连接都生成一个新的任务
        // socket 的所有权将被移到新的任务中，并在那里进行处理
        tokio::spawn(async {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{Get, Set};
    use std::collections::HashMap;

    // 使用hashMap存储redis数据
    let mut db = HashMap::new();

    // connection  对于redis的读取进行了抽象，读取的是一个一个数据帧frame(数据帧 = redis 命令+数据),而
    // 不是字节流
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT {:?}", frame);

        // 回复一个错误
        // let response = Frame::Error("unimplemented".to_string());

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 Vec<u8> 的形式
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    let v = value.clone();
                    Frame::Bulk(v.into())
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
