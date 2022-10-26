use std::collections::{BTreeSet, HashSet, BTreeMap};


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

macro_rules! set {
    ( $($item: expr), *) => {
        {// a nested scope to isolate the generated code,
        let mut s = BTreeSet::new();
        $(s.insert($item);)*
        s
        }
    };
}

macro_rules! key_value {
    ($cls:ty, $( $key:expr => $value:expr),* ) => {
        {
            let mut s = <$cls>::new();
            $(s.insert($key, $value);)*
            s
        }
    };
}

#[proc_macro]
pub fn print_info(_item:TokenStream) -> TokenStream{
    "fn print_info() {println!(\"Hello!\");}".parse().unwrap()
}

pub fn run_mcaro_main(){
    println!("two is: {}", two!());
    println!("two calc is : {}", calc!(two));
    println!("three calc is : {}", calc!(three));
    println!("repeat n times {:?}", repeat_n_times!(3, "Hello"));
    println!("status handler {:?}", stauts_handler());


    let actual = set!("a", "b", "c", "a");
    println!("actual: {:?}", actual);

    let actual_k = key_value!(BTreeMap<&str, usize>, "Hello" => 2, "word" => 1);
    println!("actual_k: {:?}", actual_k);
}

