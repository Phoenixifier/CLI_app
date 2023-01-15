use std::{env, process};

use command_line::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Error passing arguments {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = command_line::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}



