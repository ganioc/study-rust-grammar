
enum Option <T> {
    None,
    Some(T),
}

/// .
fn number_match(number: usize) -> String{
    match number{
        0|1 => "zero or one".to_string(),
        2...9 => "two to nine".to_string(),
        10 => "ten".to_string(),
        _ => "anything else".to_string()
    }
}
fn str_match(text: &str) -> String{
    match text {
        "horse" => "UNI".to_string(),
        "girl" => "Rustacean".to_string(),
        _ => "Nothing specific".to_string()
    }
}

fn tuple_match(tuple:(u32, u32, u32, u32)) -> String {
    match tuple{
        (first, _ , third, _) => format!("[{}, ..., {}, ...]", first , third)
    }
}

fn main(){
    let a_wrapped_value = Some(100_usize);

    println!("Let's do some mathing work.");


    if let Some(the_value) = a_wrapped_value{
        println!("{} was the wrapped value", the_value);
    }else{
        println!("No value was wrapped");
    }

    match a_wrapped_value {
        Some(the_value) => println!("{} was the wrapped value", the_value),
        None => println!("No value was wrapped")
    }

    println!("match 2: {}",number_match(2));
    println!("str match : {}", str_match("go"));
    println!("tuple match : {}", tuple_match((1,2,3,4)));


}