use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;

pub fn test_error_handling() {
    let result = read_username_from_file();

    match result {
        Ok(name) => println!("filename:{}", name),
        Err(_) => println!("Not get filename"),
    }

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    println!("home ip: {:?}", home);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

#[cfg(test)]
pub mod tests_all_facets {
    #[test]
    fn test_the_thing() {
        let val = 5;
        assert!(val == 5);
    }
}
