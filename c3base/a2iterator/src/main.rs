
fn main() {}

#[test]
fn test_iter1() {
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    for v in arr.iter() {
        println!("v: {}", v);
    }
    println!("arr: {:?}", arr);
}