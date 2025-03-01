fn main(){
    let numbers : Vec<i32> = vec![1, 2, 3, 4, 5];
    process_numbers(&numbers);
}


fn process_numbers(numbers : &Vec<i32>) {
    let mut sum : i32 = 0;

    for number in numbers {
        sum += number;
    }

    println!("Sum of numbers: {}", sum);

    if sum % 2 == 0 {
        println!("Sum is even");
    } else {
        println!("Sum is odd");
    }
}