use std::fs::File;
use std::io::ErrorKind;
use std::net::IpAddr;

fn main() {
    // unwrap 简而言之：成功则返回值，失败则 panic，总之不进行任何错误处理。
    // let h: IpAddr = "123".parse().unwrap(); // 因为格式不对，所以会 panic

    open_file();
    open_file2();
}


fn open_file() {
    let f = File::open("hello.txt");
    let fileObj: File;
    match f {
        Ok(file) => fileObj = file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
        }
    };
}

fn open_file2() {
    let f = File::open("hello.txt");
    let mut fileObj: Option<File> = Option::None;
    match f {
        Ok(file) => fileObj = Option::Some(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("not found");
                match File::create("hello.txt") {
                    Ok(fc) => fileObj = Option::Some(fc),
                    Err(e) => panic!("error {}", e),
                }
            }
            other_err => println!("Problem opening the file: {:?}", other_err)
        }
    };
    println!("fileObj {:?}", fileObj);
}