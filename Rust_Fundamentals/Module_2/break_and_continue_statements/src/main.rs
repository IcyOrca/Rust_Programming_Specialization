fn main() {
    for i in 1..=10 {
        // skip the even numbers
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);

        // break the loop when i is 7
        if i == 7 {
            break;
        }
    }
}
