use std::fs::read_to_string;

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Environment variable not found")]
    EnvironmentError(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn reader() -> Result<String, MyError> {
    let file = std::env::var("MARKDOWN")?;
    println!("file: {}", file);
    let content = read_to_string(file)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // MARKDOWN=/Users/li/go/src/github.com/lpxxn/rs-play/c3base/a6err/src/a.txt
        let s = reader();
        println!("s: {:?}", s);
    }
}