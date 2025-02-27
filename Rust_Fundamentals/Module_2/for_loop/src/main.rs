fn main() {
    
    // Excluding 5
    for i in 1..=10 {
        println!("{}", i);
    }

    // Including 5
    for i in 1..5 {
        println!("{}", i);
    }

    // Reverse order
    for i in (1..10).rev() {
        println!("{}", i);
    }
}
