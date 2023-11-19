use std::fs::File;
use std::net::IpAddr;

fn main() {
    // unwrap 简而言之：成功则返回值，失败则 panic，总之不进行任何错误处理。
    // let h: IpAddr = "123".parse().unwrap(); // 因为格式不对，所以会 panic

    open_file();
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
    let fileObj: File;
    match f {
        Ok(file) => fileObj = file,
        Err(error) => match error. {
            println!("Problem opening the file: {:?}", error);
        }
    };
}