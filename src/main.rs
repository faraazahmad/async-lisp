use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file.");
    
    println!("{:?}", contents);
}
