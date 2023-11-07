fn main() {
    let arr: [String; 8] = std::array::from_fn(|i| format!("{}-{}", i, i));
    println!("arr: {:?}", arr);

    let one = [1, 2, 3];
    let two:[u8; 3] = [1, 2, 3];
    let blank = [0; 3];
    let blank2: [u8; 3] = [1; 3];
    println!("one: {:?}, two: {:?}, blank: {:?}, blank2: {:?}", one, two, blank, blank2);

    for a in &arr {
        println!("a: {}", a);
    }
    for i in 0..one.len() {
        println!("one[{}]: {}", i, one[i]);
    }
}
