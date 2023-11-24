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

    #[test]
    fn test_iter3() {
        let values = vec![1, 2, 3];
        for va in values.into_iter().into_iter().into_iter() {
            println!("v: {va}");
        }
    }

    #[test]
    fn test_iter4() {
        let values = vec![1, 3, 4];
        let v1_iter = values.iter();
        println!("v1_iter: {:?}", v1_iter);
        let total: i32 = v1_iter.sum();
        //        ----- `v1_iter` moved due to this method call
        println!("total: {total}");

        // v1_iter 是借用了 values，因此 values 可以照常使用
        println!("{:?}", values);

        // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
        println!("{:?}", v1_iter);
    }
}