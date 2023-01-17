


pub fn run_function(){
    println!("run function()");
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter(){
        match name {
            &"Ferris" => println!("there is a rustacean"),
            _ => println!("Hello {}", name),
        }
    }
}