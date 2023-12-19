// cargo run --bin client

use tokio::sync::{mpsc, oneshot};
use bytes::Bytes;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    // clone a tx handle for the second f
    let tx2 = tx.clone();
    let manager = tokio::spawn(async move {
        // open one connection to the mini-redis address
        let mut client = client::connect("127.0.0.1:6378").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // ignore erros
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    // spawn two tasks, one setting a value and other querying for key that was set.
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rv) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }
        // Await the response
        let resp = resp_rv.await;
        println!("GOT (GET) = {:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rev) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        let resp = resp_rev.await;

        println!("GOT (SET) = {:?}", resp);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

/*
https://github.com/tokio-rs/website/blob/master/tutorial-code/channels/src/main.rs
 */