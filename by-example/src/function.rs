use std::mem;



pub struct Point{
    x: f64,
    y: f64,
}
impl Point{
    // static method
    fn origin() -> Point{
        Point{x:0.0, y: 0.0}
    }
    fn new(x: f64, y: f64)-> Point{
        Point{x:x, y:y}
    }
}
pub struct Rectangle{
    p1: Point,
    p2: Point,
}
impl Rectangle{
    // instance method
    fn area(&self) -> f64 {
        let Point{ x: x1, y:y1} = self.p1;
        let Point{ x:x2, y:y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} =  self.p1;
        let Point {x: x2, y: y2} =  self.p2;

        2.0 * ((x1-x2).abs() + (y1 - y2).abs())
    }
    // &mut self, self: &mut Self, 的语法糖,
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

pub struct Pair(Box<i32>, Box<i32>);

impl Pair{
    // self为self: Self的语法糖
    fn destroy(self){
        let Pair(first, second) = self;

        println!("Destroying Pair ({}, {})", first, second);

    }
}

fn apply<F>(f: F) where F: FnOnce(){
    f();
}
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32)->i32 {
    f(3)
}
// closure as output
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}
fn create_fnmut() -> impl FnMut(){
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}
fn create_fnonce() -> impl FnOnce(){
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

/* main() function */
pub fn run_function(){
    println!("run function()");
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter(){
        match name {
            &"Ferris" => println!("there is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }

    let rectangle = Rectangle{
        // 静态方法的使用, ::
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle{
        p1: Point::origin(),
        p2: Point::new( 1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // fn function (i:i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| -> i32 { i+1 };
    let closure_inferred  = | i | i+1;

    let i = 1;
    println!("function: {}", closure_annotated(i));
    println!("closure inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    let movable =  Box::new(3);

    let consume = ||{
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // consume();

    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("There're {} elements in vec", &haystack.len());

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = ||{
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");

        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2*x;
    println!("3 doubled: {}", apply_to_3(double));

    let x = 7;
    let print = || println!("{}", x);

    apply(print);

    let fn_plain =  create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

}