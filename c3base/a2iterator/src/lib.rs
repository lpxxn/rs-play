mod a1_iter;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


#[test]
fn test_enumerate() {
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    // 调用 Iterator 特征上的方法 enumerate，该方法产生一个新的迭代器，其中每个元素均是元组 (索引，值)
    for (i, v) in arr.iter().enumerate() {
        println!("i: {}, v: {}", i, v);
    }
    println!("arr: {:?}", arr);

    let map_data = {
        let mut map = std::collections::HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        map
    };
    for (k, v) in map_data.iter().enumerate() {
        println!("k: {}, v: {:?}", k, v);
    }
}

