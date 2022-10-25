
macro_rules! two{
    () => {1 + 1}
}
macro_rules! calc {
    (two) => {
      1 + 1  
    };
    (three) =>{
        1 + 2
    }
}
macro_rules! repeat_n_times {
    ($n: expr, $text: expr) => {
        (0..$n).map(|_| format!("{}", $text))
                .collect::<Vec<String>>()
    };
}

#[derive(PartialEq, Debug)]
struct Response(usize);

// Generate a function using a macro
macro_rules! handler {
    ($i: ident, $body :block) =>{
        fn $i () -> Response $body
    }
}

handler!(stauts_handler, { Response(200) });

pub fn run_mcaro_main(){
    println!("two is: {}", two!());
    println!("two calc is : {}", calc!(two));
    println!("three calc is : {}", calc!(three));
    println!("repeat n times {:?}", repeat_n_times!(3, "Hello"));
    println!("status handler {:?}", stauts_handler());
}

