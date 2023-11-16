use std::ops::{Add, Mul, Sub};

#[derive ! (Debug, PartialEq)]
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