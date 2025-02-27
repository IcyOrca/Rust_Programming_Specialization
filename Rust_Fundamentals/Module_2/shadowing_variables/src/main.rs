fn main() {
    let mut height : f32 = 190.2;
    height = height - 12.4;

    let result : &str = if height > 180.0 {
        "You are tall!"
    } else if height > 150.0 {
        "You are average!"
    } else {
        "You are short!"
    };

    println!("{}", result);

    let health = if height > 100.4 {"good"} else {"bad"};
    println!("{}", health);

    let health = if height > 100.9 {true} else {false};
    println!("{}", health);

}
