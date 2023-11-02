fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}
