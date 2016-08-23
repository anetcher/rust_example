use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, rust!");

    for argument in args {
        match argument.as_ref() {
            "1" => println!("One!"),
            _ => println!("Other!")
        }
    }
}
