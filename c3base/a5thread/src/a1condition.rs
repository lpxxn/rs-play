/*
条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行：
 */

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn condition1() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("change started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("wait");
        started = cvar.wait(started).unwrap();
    }
    println!("started");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition1() {
        condition1();
    }
}