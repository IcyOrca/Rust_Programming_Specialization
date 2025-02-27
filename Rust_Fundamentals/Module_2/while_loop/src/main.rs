use std::io::{self, Write}; // Import Write for flushing

fn main() {
    let mut x: i32 = 1;

    while x <= 10 {
        println!("{}", x);
        x += 1;
    }

    let mut input = String::new();

    while input.trim() != "exit" {
        input.clear();
        print!("Enter a number: "); 
        io::stdout().flush().unwrap(); // Ensure the prompt appears before input

        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!("You entered: {}", input.trim());
    }

    print!("\x1B[2J\x1B[1;1H"); // Clears the terminal
    io::stdout().flush().unwrap(); // Flush to apply changes immediately
    
    println!("The program has ended!");
}
