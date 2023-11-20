mod l1;
mod l2;

fn main() {
    let a = "abc";
    let b = invalid_output3();
    println!("a: {}", a);
    println!("b: {}", b);
    let c = get_str();
    println!("c: {}", c);
}


fn invalid_output() -> String {
    String::from("foo")
}

fn invalid_output2() -> &'static str {
    "foo"
}

fn invalid_output3<'a>() -> &'a str {
    "foo"
}

// return &str
fn get_str<'a>() -> &'a str {
    let s = "abcd";
    s
}