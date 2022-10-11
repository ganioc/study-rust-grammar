fn main() {
    let text: String = "hello my world!".to_string();
    println!("{}", text);
    println!("f64: from {} to {}", f64::MIN, f64::MAX);
    let my_u32: u32 = 12344;
    let a_char = 'c';
    let _byte_buffer = b"raw-byte";
    let (one,two): (u8,u8) = (1,2);
    let my_array:[u8;3] = [1,2,3];

    println!("my : {}", my_u32);
    println!("a_char: {}", a_char);
    println!("one {} two {}", one , two);
    // println!("array {}", my_array);

    let x:  f64 = 1.0*10.0;
    println!("x: {}", x);

}
