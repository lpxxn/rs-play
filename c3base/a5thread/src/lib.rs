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
}
