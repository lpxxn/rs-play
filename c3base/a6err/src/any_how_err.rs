use std::fs::read_to_string;
use anyhow::Result;


fn render() -> Result<String> {
    let s = std::env::var("MARKDOWN")?;
    println!("s: {}", s);
    let content = read_to_string(s)?;
    Ok(content)
}
#[test]
fn test_anyhow1() {
    // MARKDOWN=/Users/li/go/src/github.com/lpxxn/rs-play/c3base/a6err/src/a.txt
    let s =render();
    println!("s: {:?}", s);
    // get s error string value
    let err = s.err();
    match err {
        None => { println!("s is Ok") }
        Some(e) => { println!("s error: {:?}", e.to_string()) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let s = render()?;
        println!("s: {}", s);
        Ok(())
    }
}