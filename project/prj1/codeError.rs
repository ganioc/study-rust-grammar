trait MyTrait {
    type Associate;
    fn function_a();
    fn function_b();
}
struct MyStruct {
    pub id: u32,
}

impl MyTrait for MyStruct {
    type Associate = usize;

    fn function_a() {
        println!("a()");
    }
    fn function_b() {
        println!("b()");
    }
}

impl MyStruct {
    pub fn new(id: u32) -> Self {
        MyStruct { id }
    }
    pub fn tell(& mut self){
        println!("id: {}", self.id );
    }
}
// Automatically impl std::fmt::Debug if all members implement it
#[derive (Debug)]
struct SimpleConfiguration{
    no_threads: usize,
    verbosity: u8,
}
impl Default for SimpleConfiguration{
    fn default() -> Self{
        SimpleConfiguration{
            no_threads: 2,
            verbosity: 1,
        }
    }
}

trait Configuration{
    /// Returns the number of threads
    fn no_threads(&self) -> usize;

    /// Returns verbosity settings 0 = none, >3 = high
    fn verbosity(&self) -> u8;
}
impl Configuration for SimpleConfiguration{
    fn no_threads(&self) -> usize{
        self.no_threads
    }
    fn verbosity(&self)->u8 {
        self.verbosity
    }
}
// pub fn run_app(config: &impl Configuration){
//     // this statement creates a list of thread::Builder 
//     // instances using a range (0..n) and the map function
//     // .collect() executes the iteration (otherwise nothing
//     // happens) and returns the result as a Vec<Builder>
//     let threads: Vec<_> = (0..config.no_threads()).map(|_| std::thread::Builder::new()).collect::<Vec<std::thread::Builder>>();

//     // Verbosity levels
//     match config.verbosity(){
//         1 => println!("Threadpool set up"),
//         2 => println!("Threadpool set up with {} threads", config.no_threads()),
//         3 => println!("Threadpool set up with {} threads: {:?}", config.no_threads(), threads),
//         _ => {}
//     }
//     // return(threads); // Run the actual app
// }

enum Weekdays{
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday{ display_name: String},
    Sunday { display_name: String}
}

impl Weekdays{
    pub fn display_name(&self) -> String{
        match self{
            // the pipe (|) acts as an or chain of all variants
            Weekdays::Monday(name)
            | Weekdays::Tuesday(name)
            | Weekdays::Wednesday(name)
            | Weekdays::Thursday(name)
            | Weekdays::Friday(name) => name.clone(),
            Weekdays::Saturday{ display_name}
            | Weekdays::Sunday{ display_name} =>
                display_name.clone()
        }
    }
}
enum ApplicationError{
    Code { full: usize, short: u16},
    General (String),
    IOError (std::io::Error),
    Unknown
}
// fn something_fallible(fail: bool) -> Result<u8,u8>{
//     if !fail{
//         OK(1)
//     }else{
//         Err(0)
//     }
// }



/************************************************ */

fn main() {
    println!("Hello codeError");
    let mut elem = MyStruct::new(0x18);

    elem.tell();
    // elem as MyTrait::function_a();

    // elem.function_a();
    // elem.function_b();
    let monday = Weekdays::Monday("Monday".to_string());
    println!("{}", monday.display_name());

    let saturday = Weekdays::Saturday{
        display_name: "Saturday".to_string()
    };
    println!("{}", saturday.display_name());


}
