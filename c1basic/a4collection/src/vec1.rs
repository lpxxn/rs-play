fn vec1() {
    let arr: [i32; 3] = [1, 2, 3];
    let v = Vec::from(arr);
    vec1_is_vec(&v);
    println!("v: {:?}", v);
    let v2 = vec![1, 2, 3];
    vec1_is_vec(&v2);
    // vec!(..) 和 vec![..] 是一样的宏，宏可以使用 []、()、{} 这三种形式
    let mut v1 = Vec::new();
    for i in &arr {
        v1.push(*i);
    }
    vec1_is_vec(&v1);

    let mut v1 = vec!();
    for i in &arr {
        v1.push(*i);
    }
    vec1_is_vec(&v1);
}

fn vec_extend() {
    let mut v1 = vec![1, 2, 3];
    v1.pop();
    v1.push(4);
    let mut v2: Vec<i32> = vec!();
    v2.extend(&v1);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    assert_eq!(v1, v2);
    v2.extend([5, 6, 7].iter());
    println!("v2: {:?}", v2);
    v2.extend([8, 9, 10]);
    println!("v2: {:?}", v2);
}

fn vec_from_into() {
    let arr: [i32; 3] = [1, 2, 3];
    let v = Vec::from(arr);

    let v2 = arr.to_vec();
    assert_eq!(v, v2);
    println!("v: {:?}", v);

    let s = "hello".to_string();
    let v: Vec<u8> = s.clone().into();

    let v2 = Vec::from(s);
    assert_eq!(v, v2);
    println!("v: {:?} v2: {:?}", v, v2);

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    println!("v2: {:?}", v2);
    assert_eq!(v, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    println!("v3: {:?}", v3);
    assert_eq!(v, v3);

    // 迭代器 Iterators 可以通过 collect 方法转换为 Vec
    let v4: Vec<i32> = (0..5).collect();
    println!("v4: {:?}", v4);
    assert_eq!(v4, vec![0, 1, 2, 3, 4]);

    let v5: Vec<i32> = [0; 10].into_iter().collect();
    println!("v5: {:?}", v5);
    assert_eq!(v5, vec![0; 10]);


    let mut vec = Vec::with_capacity(10);
    vec.push(1);
    println!("vec: {:?}", vec);
    assert_eq!(vec.capacity(), 10);
}

fn vec1_is_vec(v: &Vec<i32>) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec1() {
        vec1();
    }

    #[test]
    fn test_vec_extend() {
        vec_extend();
        vec_from_into();
    }
}