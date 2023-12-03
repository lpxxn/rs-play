mod r;
mod this_err_test;

use std::{fmt, io, num};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;


#[derive(Debug)]
struct AppError {
    kind: String,
    code: i32,
    msg: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppError {{ kine: {}, code: {}, msg: {} }}", self.kind, self.code, self.msg)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            code: 2,
            msg: e.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(e: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            code: 3,
            msg: e.to_string(),
        }
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        kind: String::from("app"),
        code: 1,
        msg: String::from("some error"),
    })
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn it_works() -> Result<(), AppError> {
        match produce_error() {
            Ok(_) => println!("ok"),
            Err(e) => println!("error: {}", e),
        }


        let mut file = File::open("abc.txt")?;

        Ok(())
    }

    #[test]
    fn parse_int() -> Result<(), AppError> {
        // get current directory
        let mut path = std::env::current_dir()?;
        println!("The current directory is {}", path.display());
        // append path
        path.push("src/a.txt");


        println!("The current directory is {}", path.display());
        // path to string
        let path_str = path.to_str().unwrap();
        let mut file = File::open(path_str)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let number: usize = content.parse()?;

        Ok(())
    }
}

fn reader() -> Result<String, Box<dyn Error>> {
    let file = std::env::var("MYFILE")?;
    let source = read_to_string(file)?;
    Ok(source)
}


