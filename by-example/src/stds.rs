use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::num;
use std::rc::Rc;
use std::str;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::vec;
//
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}
fn about_str() {
    let pangram = "The quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("string: {}", string);

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says : {}", alice);
    println!("Bob says: {}", bob);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: {}", my_str);
    }

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);
    // println!("Some escaped bytes: {}", escaped);
}
fn about_vec() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
}
pub type MathResult = Result<f64, MathError>;
pub fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}
pub fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
pub fn ln(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeLogarithm)
    } else {
        Ok(x.ln())
    }
}
fn op(x: f64, y: f64) -> f64 {
    match div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}
pub fn about_result() {
    println!("op {}", op(100.0, 10.0));
}

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred What can I get for you today?"
        }
        _ => "Hi, Who is this again?",
    }
}
fn about_hash() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Dont't have Daniel's number."),
    }
}
fn about_rc() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("== rc_a is created ==");
        let rc_a = Rc::new(rc_examples);

        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("-- rc_a is cloned to rc_b ---");

            let rc_b = Rc::clone(&rc_a);
            println!("Reference count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
        }
        println!("Reference of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
}
fn about_arc() {
    println!("\nAbout Arc");
    let apple = Arc::new("The same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);

        let thr = thread::spawn(move || {
            println!("1 {:?}", apple);
        });
        let _ = thr.join();
    }
}

static NTHREADS: i32 = 10;
fn about_threads() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }
    for child in children {
        let _ = child.join();
    }
}
fn about_map_reduce() {
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    let mut children: Vec<JoinHandle<(u32)>> = vec![];

    let chunked_data = data.split_whitespace();

    println!("chunked_data: {:?}", chunked_data);

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }));
    }

    // Reduce阶段,
    let mut intermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}

fn about_channel() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} sending finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    println!("rx sequence {:?}", ids);
}
pub fn run_std() {
    println!("\nrun std()");
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    let boxed_rectangle = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    let boxed_point = Box::new(origin());

    let box_in_a_box = Box::new(boxed_origin());

    println!("Point occupies {} bytes", mem::size_of_val(&point));
    println!(
        "Rectangle occupies {} bytes in the stack",
        mem::size_of_val(&rectangle)
    );

    // width of the pointer,
    println!(
        "Boxed point occupies {} bytes in the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes in the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes in the stack",
        mem::size_of_val(&box_in_a_box)
    );
    let unboxed_point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes",
        mem::size_of_val(&unboxed_point)
    );

    about_vec();
    about_str();
    about_result();
    about_hash();
    about_rc();
    about_arc();
    about_threads();
    about_map_reduce();
    about_channel();
}
