mod a1summary;
mod a2traitbound;
mod a3largest;
mod a4file;
mod default_demo;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", multiply(1, 2));
}


fn multiply<T: std::ops::Add<Output=T> + Copy>(a: T, b: T) -> T {
    a + b
}