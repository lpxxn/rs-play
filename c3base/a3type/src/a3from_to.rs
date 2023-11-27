/*
From 特征允许让一个类型定义如何基于另一个类型来创建自己，因此它提供了一个很方便的类型转换的方式。
From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：只要实现了 impl From<T> for U，
就可以使用以下两个方法: let u: U = U::from(T) 和 let u:U = T.into()，前者由 From 特征提供，而后者由自动实现的 Into 特征提供。
需要注意的是，当使用 into 方法时，你需要进行显式地类型标注，因为编译器很可能无法帮我们推导出所需的类型。
 */

use std::{io, num};
use std::io::Read;

fn f1() {
    let s = "hello";
    let s1 = String::from(s);
    let s2 = s.to_string();
    println!("s1: {}, s2: {}", s1, s2);

    //   :String 需要显式地类型标注
    let s3: String = s.into();
    println!("s3: {}", s3);

    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    println!("i1: {}, i2: {}", i1, i2);
    assert_eq!(i1, i2);

    let i3: i32 = 'a' as i32;
    let i4: i32 = i32::from('a' as u8);
    println!("i3: {}, i4: {}", i3, i4);
    assert_eq!(i3, i4);

    let s: String = 'a'.to_string();
    let s1: String = String::from('a');
    println!("s: {}, s1: {}", s, s1);
    assert_eq!(s, s1);
}


#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/*
当执行错误处理时，为我们自定义的错误类型实现 From 特征是非常有用。这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型
 */
#[derive(Debug)]
enum CliErr {
    IoErr(io::Error),
    ParseErr(num::ParseIntError),
}

impl From<io::Error> for CliErr {
    fn from(err: io::Error) -> Self {
        CliErr::IoErr(err)
    }
}

impl From<num::ParseIntError> for CliErr {
    fn from(err: num::ParseIntError) -> Self {
        CliErr::ParseErr(err)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliErr> {
    // ? 会自动将 io::Error 转换成 CliErr, 因为我们为 CliErr 实现了 From<io::Error>，在有io::Error的时候，会自动调用 From<io::Error> 的实现
    let mut file = std::fs::File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn open_and_parse_file2(file_name: &str) -> Result<i32, CliErr> {
    let contents = std::fs::read_to_string(file_name)?;

    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        f1();
    }

    #[test]
    fn test_open_and_parse_file() {
        let result = open_and_parse_file("test.txt");
        println!("result: {:?}", result);
        // assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_from() {
        let num = Number::from(30);
        println!("My number is {:?}", num);

        let int = 5;
        let num: Number = int.into();
        println!("My number is {:?}", num);
    }

}