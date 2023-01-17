use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

pub enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64}
}

pub fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"",s),
        WebEvent::Click{x,y} =>{
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

/*
 * 有点Lisp中Cons的意思,
*/
pub enum YourList{
    Cons(u32, Box<YourList>),
    Nil, // 末节点，表明链表结束
}
use YourList::*;

impl YourList {
    pub fn new() -> YourList{
        Nil
    }
    pub fn prepend(self, elem: u32) -> YourList{
        Cons(elem, Box::new(self))
    }
    pub fn len(&self) -> u32{
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    pub fn stringify(&self) -> String{
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Number{
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self{
        Number { value: item}
    }
}
#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value %2 == 0 {
            Ok(EvenNumber(value))
        }else{
            Err(())
        }
    }
}

struct Circle{
    radius: i32
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Circle of radius {}", self.radius)
    }
}


pub fn run_types(){
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

    let circle =  Circle{ radius : 6};
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    

}