fn iterator1() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        iterator1()
    }

    #[test]
    fn test_iter2() {
        let values = vec![1, 2, 3, 4];
        {
            let result = match IntoIterator::into_iter(values) {
                mut iter => loop {
                    match iter.next() {
                        Some(x) => { println!("x: {x}") }
                        None => break,
                    }
                }
            };
        }
    }
}