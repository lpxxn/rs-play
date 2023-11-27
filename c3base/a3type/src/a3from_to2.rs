use std::{io, num};
use std::io::Read;


fn try_into1() {
    let num: i16 = 256;
    let n: u8 = match num.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error: {:?}", e.to_string());
            0
        }
    };
}


#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_into1() {
        try_into1();
    }

    #[test]
    fn test_try_into2() {
        let result = EvenNum::try_from(8);
        assert_eq!(result, Ok(EvenNum(8)));
        let result = EvenNum::try_from(5);
        assert_eq!(result, Err(()));
    }
}