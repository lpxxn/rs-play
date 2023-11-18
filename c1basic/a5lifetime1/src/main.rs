mod l1;
mod l2;

fn main() {
    println!("Hello, world!");
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