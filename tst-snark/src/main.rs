use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about,long_about = None)]
struct Args{
    #[arg(short, long)]
    name: String,

    // Number of times to greet,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}



fn main() {
    println!("Hello, world!");

    // 用这个函数来做传入参数的解析
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
