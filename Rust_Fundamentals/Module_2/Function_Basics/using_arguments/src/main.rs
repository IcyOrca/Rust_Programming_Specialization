fn sum(numbers : Vec<f32>) -> f32 {

    let mut result : f32 = 0.0;

    for number in numbers{
        result += number;
    }

    return result;
}

fn main() {
    let numbers : Vec<f32> = vec![1.3,4.5,6.7,8.9];

    println!("Sum of numbers: {}", sum(numbers));
}
