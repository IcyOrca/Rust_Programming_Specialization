fn main() {
    let message: &str = "Hello, world!";
    let weight: f32 = 100.0;

    let kilos: f32 = weight / 2.2;
    println!("{} {}", message, kilos);

    mutable_example();
}

fn mutable_example() {
    let mut message = String::from("Name: Alfredo, Height: ");

    let mut height: f32 = 180.2;
    println!("{}{}", message, height);

    height = 190.2;

    message.clear();

    println!("{}{}", message, height);
}
