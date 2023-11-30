mod a1condition;

use std::cell::RefCell;
use std::sync::{Arc, Barrier};
use std::thread;

fn th1() {
    let v = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap(); //通过调用 handle.join，可以让当前线程阻塞，直到它等待的子线程的结束
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        th1();
    }

    #[test]
    fn test_barrier1() {
        test_barrier();
    }

    #[test]
    fn test_barrier2() {
        th_local();
    }
}

// 线程屏障
// 在 Rust 中，可以使用 Barrier 让多个线程都执行到某个点后，才继续一起往后执行：
fn test_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn th_local() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    // 每个线程开始时都会拿到线程局部变量的初始值，这里是 1
// 然后通过 with 方法，可以获取到线程局部变量的值，这里是 1
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });
    // 等待子线程结束
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2); // 还是2
    });
}












