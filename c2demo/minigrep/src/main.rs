use std::{env, process};

use minigrep::Config;

fn main() {
    println!("Hello, world!");
    let arts: Vec<String> = env::args().collect();
    dbg!(&arts);

    if arts.len() < 3 {
        println!("not enough args");
        return;
    }

    // let (query, file_path) = parse_config(&arts);
    // let config = parse_config(&arts);
    let config = Config::new(&arts);
    let config = Config::build(&arts).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query: {}, file_path: {}", config.query, config.file_path);
    // run(&config).unwrap_or_else(|err| {
    //     println!("Application error: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}