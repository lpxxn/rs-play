use std::fs::read_to_string;
use anyhow::Result;


fn render() -> Result<String> {
    let s = std::env::var("MARKDOWN")?;
    println!("s: {}", s);
    let content = read_to_string(s)?;
    Ok(content)
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