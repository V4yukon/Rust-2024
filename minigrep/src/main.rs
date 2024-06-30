use std::{env, process};
use nimigrep::Config;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    // let input_parameters: Vec<String> = env::args().collect();
    // change the parameter, from &[String] to Iterator
    let config = Config::build(env::args())
        .unwrap_or_else(|err| { 
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );
        
    println!("command: {}\nfilename: {}", config.command, config.filename);
    println!("*********************\n\n");
    match config.run() {
        Ok(()) => println!("\n\nGood Job!"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
    let duration = start.elapsed();

    println!("adapt time: {:?}", duration);

    // if let Err(e) = config.clone().run() {
    //     println!("oops: {e}");
    //     process::exit(1);
    // }
    // let text = fs::read_to_string(config.filename).unwrap();
    // println!("{text}");
    
}