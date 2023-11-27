use std::fmt::Display;
use std::str::FromStr;
use std::num::ParseIntError;


#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
        // parse 方法可以将一个 String 转换成 i32 数字，这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
    // type Err = ParseIntError;
    //
    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
    //     let x_fromstr = coords[0].parse::<i32>()?;
    //     let y_fromstr = coords[1].parse::<i32>()?;
    //     Ok(Point { x: x_fromstr, y: y_fromstr })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let p = Point::from_str("(1,2)");
        let p1 = p.unwrap();
        println!("p1: {:?}", p1);
        assert_eq!(p1, Point { x: 1i32, y: 2 });
    }
}