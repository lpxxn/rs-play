//标准库提供了通道std::sync::mpsc，其中mpsc是multiple producer, single consumer的缩写，代表了该通道支持多个发送者，但是只支持唯一的接收者。
/*
之前提到，通道关闭的两个条件：发送者全部drop或接收者被drop，
要结束for循环显然是要求发送者全部drop，但是由于send自身没有被drop，
会导致该循环永远无法结束，最终主线程会一直阻塞。
 */
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn mpsc1() {
    // 创建一个通道，返回一个无组（发送者，接收者）的元组
    let (tx, rx) = mpsc::channel();
    // tx means transmitter，rx means receiver
    // 创建一个线程，用于发送消息
    thread::spawn(move || { // 需要使用move将tx的所有权转移到子线程的闭包中
        let val = String::from("hi");
        tx.send(val).unwrap();
        tx.send("hello".to_string()).unwrap();
    });
    println!("try to receive {:?} from channel", rx.try_recv()); // try_recv尝试接收一次消息，该方法并不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误：
    // recv方法会阻塞主线程执行，直到从通道中接收一个值
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}


fn mpsc2() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // 通过clone方法可以创建一个新的发送者，这样就可以有多个发送者了

    let tx2 = tx.clone();
    thread::spawn(move || {
        println!("before sleep 1");
        thread::sleep(Duration::from_secs(1));
        let val = String::from("hi tx2");
        tx2.send(val).unwrap();
        println!("after sleep 1");
        // println!("val: {}", val); // val的所有权已经转移到子线程中，所以这里会报错
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpsc1() {
        mpsc1();
    }

    #[test]
    fn test_mpsc2() {
        mpsc2();
    }
}