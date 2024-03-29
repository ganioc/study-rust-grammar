use std::error;
use std::fmt;
use std::num;
use std::{num::ParseIntError, vec};

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("Aaa!!");
    }
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

struct Person {
    job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm, I love {:?}", food),
        None => println!("Oh no, It wasn't edible."),
    }
}

#[derive(Debug)]
enum MyFood {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum MyDay {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::Sushi => None,
        _ => Some(food),
    }
}
fn have_recipe(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: MyFood) -> Option<MyFood> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: MyFood) -> Option<MyFood> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat_v2(food: MyFood, day: MyDay) {
    match cookable_v2(food) {
        Some(food) => println!("Yay, On {:?} we get to eat {:?}", day, food),
        None => println!("Oh no, We dont get to eat on {:?}?", day),
    }
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();

    first_number * second_number
}

fn my_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn my_print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error : {}", e),
    }
}

type AliasedResult<T> = Result<T, ParseIntError>;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

type MyResult<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
fn my_double_first(vec: Vec<&str>) -> MyResult<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}
fn my_double_print(result: MyResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn run_panics() {
    println!("\nHello, run panics()");

    give_princess("teddy bear");
    // give_princess("snake");

    let snake = Some("snake");
    let food = Some("chicken");

    give_commoner(snake);
    give_commoner(food);

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    println!("phonenumber: {:?}", p.work_phone_area_code().unwrap());

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    let (cordon_bleu, steak, sushi) = (MyFood::CordonBleu, MyFood::Steak, MyFood::Sushi);

    eat_v2(cordon_bleu, MyDay::Monday);
    eat_v2(steak, MyDay::Tuesday);
    eat_v2(sushi, MyDay::Wednesday);

    let twenty = multiply("10", "2");
    println!("double ten is {}", twenty);

    let tt = my_multiply("t", "20");
    println!("double ten is {:?}", tt);

    let numbers = vec!["32", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));

    let numbers2 = vec!["32", "93", "18"];
    let empty2: Vec<&str> = vec![];
    let strings2 = vec!["tofu", "93", "18"];

    // my_double_print(my_double_first(numbers2));
    // my_double_print(my_double_first(empty2));
    // my_double_print(my_double_first(strings2));

    let (numbers2, errors2): (Vec<_>, Vec<_>) = strings2
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    println!("Numbers: {:?}", numbers2);
    println!("Errors: {:?}", errors2);
}
