fn main() {

    let mut x : i32 = 1;

    loop {
        println!("{}", x);
        x += 1;

        if x > 10 {
            break;
        }
    }

    println!("Hello, world!");
}
