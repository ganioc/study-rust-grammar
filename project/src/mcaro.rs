
macro_rules! two{
    () => {1 + 1}
}

pub fn run_mcaro_main(){
    println!("two is: {}", two!());
}

