use std::ffi::CString;
use std::fmt;
use std::fmt::{Formatter, write};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 三种方式实现
impl<T: Sub<T, Output=T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

//------------------------ Self ------------------------
impl<T: Add<Output=T>> Add<Self> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// -----------------------Point<T>------------------------
impl<T: Mul<Output=T>> Mul<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_sub() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1 - p2;
        println!("p3: {:?}", p3);

        let p1 = Point { x: 1.1, y: 2.2 };
        let p2 = Point { x: 3.3, y: 4.4 };
        let p3 = p1 - p2;
        println!("p3: {:?}", p3);
    }

    #[test]
    fn test_pretty() {
        let p = Pretty("hello".to_string());
        println!("p: {}", p);
    }
}