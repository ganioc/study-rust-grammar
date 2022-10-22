
use std::io::{Read, Write};
use std::net::{ Shutdown, TcpListener, TcpStream};
use std::io::{ BufReader, BufWriter};
use std::fs::File;


pub fn read_it(){
    println!("I will read it!");
}

pub fn run_main_svr() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42380")?;
    println!("Listening to {:?}", listener.local_addr()?);
    loop{
        // a simple sequential TCP server that returns what it receives
        match listener.accept(){
            Ok ((mut stream, addr)) => {
                println!("{:?} connected", addr);
                let mut buf = vec![0;100];
                let bytes_read = stream.read(&mut buf)?;
                let bytes_written = stream.write(&buf[0..bytes_read])?;

                println!(
                    "Received {} bytes, send {} bytes.",
                    bytes_read, bytes_written
                );
                stream.shutdown(Shutdown::Both)?;
            }
            Err(e) => eprintln!("Error accepting client connection:{:?}", e),
        }
    }
}

pub fn run_main_client() -> std::io::Result<()> {
    println!("Connect to 42380");
    
    match TcpStream::connect("127.0.0.1:42380") {
        Ok(mut stream) => {
            let payload = b"Hello Rust";
            println!(
                "Connected to the server! Sending'{}'",
                String::from_utf8_lossy(payload)
            );
            let bytes_written = stream.write(payload)?;
            let mut received = String::new();
            stream.read_to_string(&mut received)?;
            println!("Received: {}", received);
            stream.shutdown(Shutdown::Both);
            Ok(())
        }
        Err(e) => {
            eprintln!("Couldn't connect to server ...");
            Err(e)
        }
    }
}

fn write(content: &str, file: impl Write) -> std::io::Result<()>{
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(content.as_bytes())?;
    Ok (())
}
fn read(file: impl Read) -> std::io::Result<String>{
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run_main_file() -> std::io::Result<()>
{
    let content = "Hello";
    {
        let file = File::create("testfile.txt")?;
        write(content, file)?;
    }
    // running out of the scope will close the file
    {
        let file = File::open("testfile.txt")?;
        let outcome = read(file)?;
        assert_eq!(content, outcome);
    }
    // running out of the scope will close the file
    Ok(())
}

#[derive(Debug)]
struct Account{
    name: String,
    password: [u8;5] // use 32 for a 256 bit hash(SHA-256)
}

pub fn run_formatting(){
    println!("My name is {name}", name="Arthur Dent");
    println!("Pi = {pi:30.10}", pi=std::f64::consts::PI);
    println!("blog.x{:x}.xyz", 1535);
    println!("{:?}", vec!["hello", "world"]);

    let me = Account{ name: "Arthur".to_string(),
            password: [0;5]};
    let output = format!("{:?}", me);

    assert_eq!(output, "Account { name: \"Arthur\", password: [0, 0, 0, 0, 0] }");

}

#[allow(dead_code)]
use std::fmt;
struct YAccount{
    name: String,
    password: [u8;5]
}
impl fmt::Display for YAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Account ({name})", name = self.name)
        // Ok(())
    }
    
}

pub fn run_main_account(){
    let me = YAccount{ name: "Arthur".to_string(),
        password: [0;5]};
    let output = format!("{}", me);
    assert_eq!(output, "Account (Arthur)");
}

