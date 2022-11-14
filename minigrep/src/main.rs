use std::{env,  process};

use minigrep::{Config, run};

// % cargo run -- searchstring poem.txt

fn main() {
    println!("Hello, world! ====>");

    minigrep::shirt_run();

    let args: Vec<String> = env::args().collect();

    // let config: Config = parse_config(&args);
    // let config: Config = Config::new(&args);
    let config =  Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);


    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("with text:\n{contents}");
}

// fn parse_config(args: &[String]) -> Config {
//     // dbg!(args);

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

