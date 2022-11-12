pub fn test_variable() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces is {}", spaces);

    let t: bool = true;
    println!("t is a boolean value : {}", t);
    let tup: (i32, f64, u8) = (500, 13.3, 1);
    println!("tup is {:?}", tup);
    println!("tup 0 is {}", tup.0);

    let y = || {
        let x = 3;
        x + 2
    };
    println!("the value of y is : {}", y());

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value of a is : {element}");
    }
    for number in (1..4).rev() {
        println!("{number}");
    }
}
