fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("你好世界");
    let us1 = utf8_slice::from(s.as_str(), 2);
    println!("{}", us1);
    println!("len: {}", utf8_slice::len(&s[..]));
}
