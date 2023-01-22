use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

/*
 * 有点Lisp中Cons的意思,
*/
pub enum YourList {
    Cons(u32, Box<YourList>),
    Nil, // 末节点，表明链表结束
}
use YourList::*;

impl YourList {
    pub fn new() -> YourList {
        Nil
    }
    pub fn prepend(self, elem: u32) -> YourList {
        Cons(elem, Box::new(self))
    }
    pub fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn create_box() {
    let _box1 = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped.");
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str' 是一个对分配在只读内存区的字符串的引用,
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}
fn new_edition(book: &mut Book) {
    book.year = 2020;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x;
}

fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}
fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}
fn elided_pass(x: &i32) -> &i32 {
    x
}
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

pub fn run_types() {
    let my_str = "hello";
    let my_string = String::from(my_str); // str to String,
    println!("my_string: {}", my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num2: Number = int.into();
    println!("My number 2 is {:?}", num2);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let _x = ToDrop;
    println!("Made a ToDrop, wait it to destruct.");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    // 这个是在数据结构中，解析成员变量
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // println!("The person struct is {:?}", person);
    println!("The person's age from person struct is {}", person.age);

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has coordinates: ({},{},{})",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    println!("mutable_tuple: {:?}", mutable_tuple);

    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("now tuple is {:?}", mutable_tuple);
    let (four, nine) = (4, 9);
    println!("four {} nine {}", four, nine);
    failed_borrow();

    let x = 7;
    let y = 9;
    print_one(&x);

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    let x_elided = 3;
    elided_input(&x_elided);
    annotated_input(&x_elided);

    println!("elided_pass: {}", elided_pass(&x_elided));
    println!("annotated_pass: {}", annotated_pass(&x_elided));

    println!("End of types\n\n");
}
