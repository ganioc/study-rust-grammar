use std::io::Result;

type ConnectionFactoryFn<T> = Box<dyn Fn(host:String) -> Result<T>>;


fn print_hex(data: Box<u128>){
    println!("Your data in hex: 0x{:x}", *data);
}



trait Greeter{
    fn hello(&self){
        println!("Hello!");
    }
}


struct Person;
impl Greeter for Person {}
fn say_hello(who: Box<dyn Greeter>){
    who.hello();
}



pub fn run_main_box(){
    print_hex(Box::new(1535));
    let p = Box::new(Person{});
    say_hello(p);
}
