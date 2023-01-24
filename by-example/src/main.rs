mod format;
use crate::associate::run_associate;
use crate::format::{reverse, MyList};
use crate::macros::run_macros;
use crate::panics::run_panics;
use format::{Deep, MinMax, Person, Structure};

mod macros;
mod traits;
mod types;
use crate::function::run_function;
use crate::modules::run_modules;
use crate::traits::run_trait;
use crate::types::{inspect, run_types, WebEvent, YourList};

mod associate;
mod function;
mod modules;
mod panics;

use std::mem;

fn main() {
    println!("Hello, world!");
    println!("Let's do it!");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Blob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("{} of 0b{:b} people", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{:?} months", 11);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {} will print!", Structure(4)); // 美化了输出结果,
    println!("Now Deep {:?} will print!", Deep(Structure(17)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);

    let big_range = MinMax(10, 100);
    println!("The big_range is {big}.", big = big_range);

    let v = MyList(vec![1, 2, 3]);
    println!("{}", v);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let mut list = YourList::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    run_types();

    run_function();

    run_modules();

    run_associate();

    run_trait();

    run_macros();

    run_panics();
}
