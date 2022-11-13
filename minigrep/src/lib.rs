use std::error::Error;
use std::fs;
use std::env;

mod shirt;
pub use shirt::shirt_run;

/// Struct Config
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    fn new(args: &[String]) -> Config {
        // dbg!(args);

        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = true;

        Config { query, file_path , ignore_case}
    }
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() <3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, file_path, ignore_case})
    }
}

/// run
/// # main run method
/// 
pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    // let contents =
    // fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let contents =
    fs::read_to_string(config.file_path)?;

    println!("\nWith text:\n{contents}");

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };

    println!("Search results:");
    for line in results{
        println!("{line}");
    }

    Ok(())
}

// Comments
/// Search a query string in contents.
/// 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn iterator_demo(){
        let v1 = vec![1,2,34,5];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        println!("v2: {:?}", v2);

    }
}