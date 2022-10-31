include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));



use std::collections::{BTreeMap, HashSet};

mod front_of_house;
mod back_of_house;
mod io;
mod mcaro;
mod boxheap;
mod threading;
mod myasync;

use back_of_house::print_back_house;
use front_of_house::{ hosting::inner_hosting};
use io::{ read_it, run_main_client, run_main_svr, run_main_file, run_formatting, run_main_account};
use std::env;
use mcaro::run_mcaro_main;

use procmacro::{print_info};
use procmacro::HelloFn;
use boxheap::run_main_box;
use threading::run_main_threading;
use myasync::run_main_async;

#[derive(HelloFn)]
struct Greeter;
use procmacro::inspect;

#[inspect]
use std::collections::HashMap;
#[inspect(stdout)]
fn abc() -> usize {
    let a = 10;
    100 + 10
}

fn main() {
    println!("Hello, world!");
    let str_nonsense: String = "Nonsense".to_string();

    let mut arguments = Vec::new();

    let path = env::var("PATH").expect("PATH variable not found.");
    let paths = path.split(".").collect::<Vec<_>>();
    for item in paths {
        println!("{}", item);
    }


    println!("arguments len: {}", arguments.len());

    for argument in std::env::args() {
        println!("*** args = {}", argument);
        arguments.push(argument);
    }
    if arguments.len()< 2 {
        arguments.push(str_nonsense);
    }
    println!("1st argument: {}", arguments[1]);

    let a_set : HashSet<_> = vec![1,2,3,4].into_iter().collect();
    let a_map: BTreeMap<_,_>=vec![("one",1),("two",2),("three",3),
        ("four",4)].into_iter().collect();

    let only_one: Option<usize> = vec![1,2,3,4].into_iter().next();

    print_back_house();
    inner_hosting::inner_serve();

    // print_prj1();
    read_it();

    run_mcaro_main();

    println!("print_info: {}", "print_info");
    print_info!();
    println!("print_info: {:?}", &print_info());

    let b = Greeter{};
    println!("Hi: Greeter: {:?}", &b.hello());

    run_main_box();

    run_main_threading();

    run_main_async();

    if arguments[1].eq("client") {
        println!("It's a client");
        run_main_client();
    }else if arguments[1].eq("server") {
        println!("It's a server");
        run_main_svr();
    }else if arguments[1].eq("file") {
        println!("Running file operations");
        run_main_file();
    }else if arguments[1].eq("formatting") {
        println!("Running file formatting");
        run_formatting();
        run_main_account();
    }
    else{
        println!("Unknown arguments: {}", arguments[1]);
    }

    unsafe{
        add(1,2);
        multiply(2,2);
    }

}
