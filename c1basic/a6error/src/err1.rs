use std::env;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let current_dir = env::current_dir();
    // check current_dir is Ok or Err
    let mut current_dir = match current_dir {
        Ok(dir) => dir,
        Err(e) => return Err(e),
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


    let f = File::open(full_path_str);

    let mut f = match f {
        Ok(file) => file,
        // 打开失败
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(a) => {
            println!("a: {}", a);
            return Ok(s);
        }
        // 将错误向上传播
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_open_file1() {
        let rev = read_username_from_file();

        println!("rev {:?}", rev);
    }
}