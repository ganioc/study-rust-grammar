use std::io::Result;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;
use std::fmt;
use std::sync::Arc;
use std::thread;


// 这个地方总是会出问题，不知道如何修改,
// type ConnectionFactoryFn<T> = Box<dyn Fn(host:String) -> Result<T>>;


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

struct RcBox<T: ?Sized>{
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}

// struct Rc<T: ?Sized>{
//     ptr: NonNull<RcBox<T>>,
//     phantom: PhantomData<RcBox<T>>,
// }


#[derive(Debug)]
struct SharedData(Vec<u8>);
impl fmt::Display for SharedData{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "SharedData: {}", String::from_utf8_lossy(&self.0))
    }
}
struct  ApplicationData{
    cache: Rc<SharedData>
}



pub fn run_main_box(){
    print_hex(Box::new(1535));
    let p = Box::new(Person{});
    say_hello(p);

    let cache = Rc::new(SharedData(b"DATADATADATA".to_vec()));
    let app_data_1 = ApplicationData{ cache: cache.clone()};
    let app_data_2: ApplicationData = ApplicationData { cache: cache.clone()} ;
    drop(cache);
    println!("App data 1: {}", app_data_1.cache);
    println!("App data 2: {}", app_data_2.cache);
    // thread::spawn(move || {
    //     println!("App data 2: {}", app_data_2.cache);
    // }).join().unwrap();

}
