use std::io::{self, Write}; // Import Write for flushing

fn main() {

    let mut i_name: String = String::new();
    
    print!("Please, enter your name: ");
    
    io::stdout().flush().unwrap(); // Ensure the prompt appears before input
    io::stdin().read_line(buf: &mut i_name).expect(msg: "Failed to read line");

    match i_name.trim() {
        "Alice" => println!("Hello, Alice!"),
        "Bob" => println!("Hello, Bob!"),
        _ => println!("Hello, {}!", i_name.trim()),
    }
}
