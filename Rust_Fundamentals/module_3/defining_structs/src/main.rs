#[derive(Debug)]  // Allows using "{:?}" in println!
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first_name: &str, last_name: &str, age: u8) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }
}

fn main() {
    let person = Person::new("Nathan", "Godoy", 21);

    println!("{:?}", person);  // Prints entire struct
    println!("{:#?}", person);  // Prints entire struct, but prettier

    println!("First name: {}", person.first_name);
    println!("Last name: {}", person.last_name);
    println!("Age: {}", person.age);
}
