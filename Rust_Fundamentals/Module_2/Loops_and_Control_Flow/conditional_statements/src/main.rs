fn main() {

    let mut maybe_number = Some(4534);
    maybe_number = None;

    if let Some(number) = maybe_number {
        println!("The number is: {}", number);
    }else{
        println!("There is no number!");
    }

    println!("Hello, world!");
}
