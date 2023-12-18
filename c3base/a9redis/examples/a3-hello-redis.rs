use std::collections::HashMap;
use std::sync::{Arc};
use tokio::net::{TcpListener, TcpStream};
// use std::sync::{Mutex, MutexGuard};
use tokio::sync::Mutex;
use mini_redis::{Command, Connection, Frame};
use bytes::Bytes;

type SharedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

fn new_shared_db(num_shards: usize) -> SharedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {}