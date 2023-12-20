use std::{env, fs};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, Box<dyn Error>> {
    // let full_path_str = current_hello_txt_path()?;
    let full_path_str = current_hello_txt_path();
    let full_path_str = match full_path_str {
        Ok(path) => path,
        Err(e) => return Err(e),
    };
    println!("full_path_str: {}", full_path_str);

    let f = File::open(full_path_str);

    let mut f = match f {
        Ok(file) => file,
        // 打开失败
        Err(e) => return Err(Box::new(e)),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(a) => {
            println!("a: {}", a);
            return Ok(s);
        }
        // 将错误向上传播
        Err(e) => Err(Box::new(e)),
    }
}

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

/*
上面代码中 File::open 报错时返回的错误是 std::io::Error 类型，但是 open_file 函数返回的错误类型是 std::error::Error 的特征对象，可以看到一个错误类型通过 ? 返回后，变成了另一个错误类型，这就是 ? 的神奇之处。

根本原因是在于标准库中定义的 From 特征，该特征有一个方法 from，用于把一个类型转成另外一个类型，? 可以自动调用该方法，然后进行隐式类型转换。因此只要函数返回的错误 ReturnError 实现了 From<OtherError> 特征，那么 ? 就会自动把 OtherError 转换为 ReturnError
 */

fn read_username_from_file2() -> Result<String, Box<dyn Error>> {
    let full_path_str = current_hello_txt_path()?;
    let mut f = File::open(full_path_str)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_text3() -> Result<String, Box<dyn Error>> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_text4() -> Result<String, Box<dyn Error>> {
    let full_path_str = current_hello_txt_path()?;
    Ok(fs::read_to_string(full_path_str)?)
}

fn read_text5() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


// 当前目录
fn current_hello_txt_path() -> Result<String, Box<dyn Error>> {
    let current_dir = env::current_dir();
    // check current_dir is Ok or Err
    let mut current_dir = match current_dir {
        Ok(dir) => dir,
        Err(e) => return Err(Box::new(e)),
    };
    current_dir.push("src/hello.txt");
    let mut full_path_str = "";

    if let Some(full_path) = current_dir.to_str() {
        println!("full_path: {}", full_path);
        full_path_str = full_path;
    }
    println!("full_path_str: {}", full_path_str);

    if let Ok(current_exe) = env::current_exe() {
        println!("current_exe: {:?}", current_exe);
    } else {
        println!("current_exe: error");
    }

    Result::Ok(full_path_str.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_open_file1() {
        let rev = read_username_from_file();

        println!("rev {:?}", rev);
    }

    #[test]
    fn test_open_file2() {
        let rev = read_username_from_file2();

        println!("rev {:?}", rev);
    }

    #[test]
    fn test_open_file3() {
        let rev = read_text3();

        println!("rev {:?}", rev);
    }

    #[test]
    fn test_open_file4() {
        let rev = read_text4();

        println!("rev {:?}", rev);

        let rev = read_text5();
        println!("rev {:?}", rev);
    }
}