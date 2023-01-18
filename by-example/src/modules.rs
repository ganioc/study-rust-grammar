
use std::fmt::{Debug, Display};

pub struct OpenBox<T>{
    pub contents: T,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ClosedBox<T>{
    contents: T,
}

impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T>{
        ClosedBox { contents: contents, }
    }
}
#[derive(Debug)]
struct SingleGen<T>(T);

struct SGen<T>(T);
fn generic<T>(_s: SGen<T>){}

struct Val { val: f64, }
struct GenVal<T> { gen_val: T }

impl Val{
    fn value(&self) -> &f64 { &self.val }
}
impl <T> GenVal<T>{
    fn value(&self) -> &T { &&self.gen_val }
}

fn compare_prints<T: Debug + Display> (t: &T){
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`",t);
}
fn compare_types<T: Debug, U:Debug>(t: &T, u: &U){
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

trait PrintInOption{
    fn print_in_option(self);
}

impl<T> PrintInOption for T where 
    Option<T> : Debug{
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
}

struct Container(i32, i32);

trait Contains<A, B>{
    fn contains(&self, _:&A, _:&B) -> bool;
    fn first(&self) -> i32; // 未显式地要求'A 或B,
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container{
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool{
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container:&C) -> i32 where
    C: Contains<A, B> {
        container.last() - container.first()
    }

// trait Contains2 {
//     type A;
//     type B;

//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

// }
// fn difference2<C: Contains2>(container: &C) -> i32{
//     container
// }

pub fn run_modules(){
    println!("\nrun modules()");
    let open_box = OpenBox{ contents: "public information"};
    println!("open box contains: {}", open_box.contents);
    let closed_box = ClosedBox::new("classifed info");

    println!("closed box contains: {:?}", closed_box);

    let a :SingleGen<char> = SingleGen('A');
    println!("a is {:?}", a);

    generic::<char>(SGen('b'));

    let string = "words";
    let array = [1,2,3];
    let vec = vec![1,2,3];

    compare_prints(&string);
    compare_types(&array, &vec);

    vec.print_in_option();

}