pub mod point {
    #[derive(Debug)]
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }
    /*
    使用泛型参数前，依然需要提前声明：impl<T>，只有提前声明了，我们才能在Point<T>中使用它，
    这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。需要注意的是，这里的 Point<T> 不再是泛型声明，
    而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point。
     */
    impl<T, U> Point<T, U> {
        pub fn new(x: T, y: U) -> Point<T, U> {
            Point { x, y }
        }
        pub fn new2(x: T, y: U) -> Self {
            Point { x, y }
        }

        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    impl Point<f32, f32> {
        /*这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，
        而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。这个方法计算点实例与坐标(0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。
        这样我们就能针对特定的泛型类型实现某个特定的方法，对于其它泛型类型则没有定义该方法。
         */
        pub fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
}

#[cfg(test)]
mod test_point {
    use std::arch::aarch64::int32x2_t;
    use std::convert::Infallible;
    use std::str::Chars;
    use super::point::Point;

    #[test]
    fn test_point() {
        let p = Point::new(1, 2);
        println!("p: {:?}", p);
        let p = Point::new2(12, "string".to_string());
        println!("p: {:?}", p);
    }

    #[test]
    fn test_try_into() {
        let a: i32 = 10;
        let b: u16 = 100;

        let b_ = b.try_into()
            .unwrap();

        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }
}