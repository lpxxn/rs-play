use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{}", contents);
    Ok(contents)
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {
            query,
            file_path,
        }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // return Err("not enough args");
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file_path,
        })
    }
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {
        query,
        file_path,
    }
}