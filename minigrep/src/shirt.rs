// pub mod myclosure{
//     let expensive_closure = |num:u32| -> u32 {
//         println!("Calculating slowly ...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
// }

// enum List{
//     Cons(i32, List),
//     Nil,
// }

use std::ops::Deref;

// use crate::List::{Cons, Nil};
enum List{
    Cons(i32, Box<List>),
    Nil,
}
// use crate::List::{Cons, Nil};


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut mut_red = 0;
        let mut mut_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => mut_red += 1,
                ShirtColor::Blue => mut_blue += 1,
            }
        }
        if mut_red > mut_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// to hodl values of any type, 
struct MyBox<T>(T);
impl <T> MyBox<T> {

    fn new(x: T)-> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer{
    data: String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data`{}`", self.data);
    }
}

pub fn shirt_run() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1, 2, 3];
    println!("Before defining closure : {:?}", list);

    // thread::spawn(move || println!("From thread : {:?}", list))
    // .join()
    // .unwrap();

    // let only_borrows = ||println!("From closure: {:?}", list);

    let mut borrows_mutably = || list.push(9);

    // println!("Before calling closure : {:?}", list.clone());

    // only_borrows();
    borrows_mutably();

    println!("After calling clousure: {:?}", list);

    let mut list_rec = [
        Rectangle {
            width: 10,
            height: 2,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list_rec.sort_by_key(|r| r.width);
    println!("{:#?}", list_rec);

    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got : {}", val);
    }

    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1, Box::new(Cons(2,Box::new(Nil))));

    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5, *y);

    let z = MyBox::new(x);
    assert_eq!(5, *z);

    let c = CustomSmartPointer{
        data: String::from("custom smart pointer"),
    };
    let d = CustomSmartPointer{
        data: String::from("Custom smart pointer No. 2"),
    };
    println!("custom smart pointer created");



}
