pub mod point {
    #[derive(Debug)]
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }
    impl<T, U> Point<T, U> {
        pub fn new(x: T, y: U) -> Point<T, U> {
            Point { x, y }
        }
        pub fn new2(x: T, y: U) -> Self {
            Point { x, y }
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::point::Point;
    #[test]
    fn test_point() {
        let p = Point::new(1, 2);
        println!("p: {:?}", p);
        let p = Point::new2(12, "string".to_string());
        println!("p: {:?}", p);
    }
}