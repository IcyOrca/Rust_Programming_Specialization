fn main() {
    let proceed : bool = true;

    if proceed {
        println!("Proceeding...");
    } else {
        println!("Not proceeding...");
    }

    let height: f32 = 180.2;

    if height > 180.0 {
        println!("You are tall!");
    } else if height > 150.0 {
        println!("You are average!");
    } else {
        println!("You are short!");
    }

}
