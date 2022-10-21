use std::mem;

fn how_large(s: &String){
    println!("Size of val: {} bytes", mem::size_of_val(s));
    println!("Size of type: {} bytes", mem::size_of::<& String>());
}

fn complicated_len() -> impl Fn() -> usize {
    let s = String::from("Hello World");
    move || {
        s.len()
    }
}
fn join_into(target: &mut String, separator: char, list: &[&str]){
    let mut is_first = true;
    for elem in list{
        if !is_first{
            target.push(separator);
        }else{
            is_first = false;
        }
        target.push_str(elem);
    }
}
struct Fraction{
    pub numerator: usize,
    pub demoninator: usize
}
impl Clone for Fraction{
    fn clone(&self) -> Self{
        Fraction{
            numerator: self.numerator.clone(),
            demoninator: self.demoninator.clone(),
        }
    }
}

fn main(){
    let x = 10_u32; // 在一个scope里let就可以了,
    let owner_a = String::from("girl");
    let owner_b = owner_a;

    {
        let owner_c = String::from("horse");
    }
    let owner_d = 18_usize;
    //println!("owner_a {}", owner_a);
    println!("owner_b {}", owner_b);
    println!("owner_d {}", owner_d);

    how_large(&String::from("Hello World"));
    
    let len_fn = complicated_len();
    println!("{}", len_fn());

    let a = String::from("Rust");
    let x = a + "acean"; // a has benn moved here, 

    assert_eq!(x, "Rustacean");

    let todo = vec!["Laundry", "Dishes", "Sports"];
    let mut target = String::new();
    join_into(&mut target, '-', &todo);

    assert_eq!(target, "Laundry-Dishes-Sports");

}