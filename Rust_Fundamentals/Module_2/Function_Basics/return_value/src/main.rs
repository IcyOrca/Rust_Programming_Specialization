fn split_string(string: String, delimiter: char, field: usize) -> String {
    
    // Split the string into parts and collect them into a vector
    let parts: Vec<&str> = string.split(delimiter).collect();

     // Vec.get() returns an Option<&T>, avoiding potential out-of-bounds access
     // When you use .get(index) on a Vec<&str>, it returns Option<&(&str)>, which simplifies to Option<&&str>
     // The .copied() method is used to convert Option<&&str> to Option<&str>
    let result: Option<&str> = parts.get(field).copied();

    return match result {
        Some(result) => result.to_string(),
        None => "N/A".to_string(), // Safe default instead of panic
    }
}

fn main() {
    let chunk : String = split_string(string -> "Abelha,Abacaxi,Azul".to_string(), ',', 2);
    println!("Chunk: {}", chunk); 
}