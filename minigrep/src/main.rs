use std::{env, fs};

// % cargo run -- searchstring poem.txt

fn main() {
    println!("Hello, world! ====>");
    let args: Vec<String> = env::args().collect();

    // dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("with text:\n{contents}");
}
