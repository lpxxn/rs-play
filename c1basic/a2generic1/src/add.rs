use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::*;
    #[test]
    fn test_point_add() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1.add(p2);
        println!("p3: {:?}", p3);

        let p1 = Point { x: 1.1, y: 2.2 };
        let p2 = Point { x: 3.3, y: 4.4 };
        let p3 = p1.add(p2);
        println!("p3: {:?}", p3);
    }
}