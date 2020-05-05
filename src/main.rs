mod reader;
mod token;

use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--src" {
        let file = fs::read_to_string(&args[2]).expect("File doesn't exist!");

        let mut reader = reader::Reader::new(file);

        reader.tokenize();

        println!("{}\n", reader.src);
        print!("{:?}", reader.tokens);
    } else {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong...");

        let mut reader = reader::Reader::new(input);

        reader.tokenize();
        print!("\n{:?}", reader.tokens);
    }
}
