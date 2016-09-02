use std::env;
mod basic;
mod hello_world;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, Rust!");

    if args.len() <= 1 { print_usage(); }

    for i in 0..args.len() {
        match args[i].as_ref() {
            "fac" => println!("{}", basic::basic::fac(args[i+1].parse::<i32>().unwrap())),
            _ => println!("")
        }
    }
}

fn print_usage() {
    println!("Usage!");
}
