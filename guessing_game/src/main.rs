extern crate ferris_says;

mod variable;
use variable::test_variable;

use ferris_says::say;
use rand::Rng;
// use std::cmp::Ordering;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    // loop{
    //     println!("Please input your guess:");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     // let guess: u32 = guess.trim().parse().expect("Please type in a number!");

    //     let guess: u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed : {guess}");
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         },
    //     }
    // }

    test_variable();

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
