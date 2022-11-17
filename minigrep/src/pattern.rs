use core::fmt;


trait OutlinePrint: fmt::Display{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len+4));
    }
}

struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {
    
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn pattern_run(){
    println!("pattern_run()");

    let favorite_color: Option<&str> = None; //Some("Red");
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using favourite color , {color} , as the background.");
    } else if is_tuesday{
        println!("Tuesday is green day");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the background color");
        }else{
            println!("using orange as the background color");
        }
    }else{
        println!("using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for(index,value) in v.iter().enumerate() {
        println!("{} is at index {}", value , index);
    }

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let point = Point { x: 1, y: 3,};
    point.outline_print();

    let w = Wrapper(vec![String::from("hello"),
        String::from("world"),
        String::from("!")]);

    println!("w = {}", w);


}

