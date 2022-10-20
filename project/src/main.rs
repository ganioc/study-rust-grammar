use std::collections::{BTreeMap, HashSet};

mod front_of_house;
mod back_of_house;


use back_of_house::print_back_house;
use front_of_house::{hosting, hosting::inner_hosting};


fn main() {
    println!("Hello, world!");

    let a_set : HashSet<_> = vec![1,2,3,4].into_iter().collect();
    let a_map: BTreeMap<_,_>=vec![("one",1),("two",2),("three",3),
        ("four",4)].into_iter().collect();

    let only_one: Option<usize> = vec![1,2,3,4].into_iter().next();

    print_back_house();
    inner_hosting::inner_serve();

    // print_prj1();
    
}
