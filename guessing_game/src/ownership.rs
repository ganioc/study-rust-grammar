#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum Option<T> {
    None,
    Some(T),
}

pub fn test_ownership() {
    println!("Test ownership");

    let mut s = String::from("hello world");
    s.push_str(", let's go!");
    println!("{}", s);

    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("x is now : {}", x);

    let s1 = String::from("hello world!");
    let hello = &s1[0..5];
    let world = &s1[6..11];

    let width1 = 30;
    let height1 = 50;
    println!("The area is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);

    println!("The area is {} square pixels", area1(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:?}", rect2);

    dbg!(&rect2);

    println!("area is {}", rect2.area());

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let some_number = Some(5);
    let some_char = Some('e');
    // let absent_number = None;

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("v is {:?}", v);

    let v1 = vec![1, 2, 3, 4, 5];
    let third = &v1[2];
    println!("Third element is {}", third);

    let third = v1.get(2);
    match third {
        Some(third) => println!("Third ele is {}", third),
        None => println!("No third ele"),
    }

    for i in &v1 {
        println!("{}", i);
    }
    // panic!("Crash!!");
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area1(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("integer {}", some_integer);
}
