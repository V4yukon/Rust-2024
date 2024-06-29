use std::{env, process};
use nimigrep::Config;

fn main() {
    let input_parameters: Vec<String> = env::args().collect();

    let config = Config::build(&input_parameters)
        .unwrap_or_else(|err| { 
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );
        
    println!("command: {}\nfilename: {}", config.command, config.filename);
    println!("*********************\n\n");
    match config.run() {
        Ok(()) => println!("\n\ngood job"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }

    // if let Err(e) = config.clone().run() {
    //     println!("oops: {e}");
    //     process::exit(1);
    // }
    // let text = fs::read_to_string(config.filename).unwrap();
    // println!("{text}");
    
}