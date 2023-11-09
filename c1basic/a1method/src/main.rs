fn main() {
    let mut rect1 = Rectangle::new(30, 50);
    println!("rect1 is {:?}", rect1);
    println!("rect1 area {}", rect1.area());
    rect1.grow(10, 10);
    println!("grow rect1 {:?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn new2(width: u32, height: u32) -> Rectangle {
        Self { width, height }
    }
}

impl Rectangle {
    // `&self` 是 `self: &Self` 的语法糖
    // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn grow(&mut self, w: u32, h: u32) {
        self.width += w;
        self.height += h;
    }
}