use std::env;

#[allow(non_snake_case)] 

fn main() {
    let args: Vec<String> = env::args().collect();

    let filePath = &args[1];

    println!("In file {}", filePath);
}