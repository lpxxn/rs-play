mod test_large;
mod point;
// 这比较好玩，r#const
mod r#const;

fn main() {
    println!("add i32 {}", add(1, 2));
    println!("add f64 {}", add(1.12, 2.23));
    let p = point::point::Point{x: 1, y: 2};
    println!("p: {:?}", p);
    test_large::abc::hello();
}

fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

