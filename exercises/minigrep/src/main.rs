use std::env;
use std::process;

use minigrep::Config;

fn main() {    
    // let args:Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


    // run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
}
