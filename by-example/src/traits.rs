use std::iter;
use std::ops;
use std::vec::IntoIter;

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name);

            self.naked = true;
        }
    }
}
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah?"
        } else {
            "baaaah!"
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        // 从self 取得属性，存入inches,
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

struct Unicorn {}
struct Cow {}

trait Animal2 {
    fn noise(&self) -> &'static str;
}

impl Animal2 for Unicorn {
    fn noise(&self) -> &'static str {
        "Baaaah!"
    }
}
impl Animal2 for Cow {
    fn noise(&self) -> &'static str {
        "moooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal2> {
    if random_number < 0.5 {
        Box::new(Unicorn {})
    } else {
        Box::new(Cow {})
    }
}

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

struct Droppable {
    name: &'static str,
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

trait Person {
    fn name(&self) -> String;
}
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

trait UsernameWidget {
    fn get(&self) -> String;
}
trait AgeWidget {
    fn get(&self) -> u8;
}
struct Form {
    username: String,
    age: u8,
}
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn run_trait() {
    println!("\nrun_trait()");
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let _one_second = Seconds(1);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);

    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d: Droppable = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");

    // drop(_a);

    println!("end of the main function.");

    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!(">{}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let nil = Nil;
    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // 实例化 Pair,
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    // println!("original: {:?}", pair);
    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    println!("clone: {:?}", cloned_pair);

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);
}
