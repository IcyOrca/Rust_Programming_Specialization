use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

fn main() {
    let file = File::open("nothing.txt").or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("nothing.txt")
        } else {
            Err(error)
        }
    }).expect("There was a problem opening or creating the file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }
}
