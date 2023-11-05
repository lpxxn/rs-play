fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("你好世界");
    let us1 = utf8_slice::from(s.as_str(), 2);
    println!("{}", us1);
    println!("len: {}", utf8_slice::len(&s[..]));

    println!("index 1 {}", utf8_slice::slice(&s, 0, 1));

    str_1();
}


fn str_1() {
    // 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
    let s: &str = "hello world";

    // 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
    let s: Box<str> = "hello world".into();
    greetings(&s);

    let s: Box<&str> = Box::new("hello world");
    greetings(*s);
}

fn greetings(s: &str) {
    println!("hello {}", s);
}