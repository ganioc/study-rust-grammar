
use std::io::{Read, Write};
use std::net::{ Shutdown, TcpListener, TcpStream};


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