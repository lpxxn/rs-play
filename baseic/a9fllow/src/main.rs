fn main() {
    let arr: [String; 8] = std::array::from_fn(|i| format!("{}-{}", i, i));

    for a in &arr {
        println!("a: {}", a);
    }
    for i in 0..arr.len() {
        // println macro will call Display trait, do not take ownership
        println!("arr[{}]: {}", i, arr[i]);
    }

    println!("arr: {:?}", arr);
    for (i, a) in arr.iter().enumerate() {
        println!("arr[{}]: {}", i, a);
    }
    println!("arr: {:?}", arr);

    let condition = true;
    let number = if condition {
        5
    } else {
        8
    };
    println!("The value of number is: {}", number);
}
