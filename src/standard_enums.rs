use std::fs::read_to_string;

//Option and Result enums
//They help handle null values  Ooption enums
//Error handling enums  Result enums

fn main() {
    let result: Option<i32> = find_first_a(String::from("This is the first day at work"));

    match result {
        Some(index) => println!("This is the index {}", index),
        None => println!("No value was found for the letter"),
    };

    match read_file("options.rs".to_string()) {
        Ok(file_content) => println!("{}", file_content),
        Err(error) => println!("{}", error),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

fn read_file(filename: String) -> Result<String, String> {
    let result = read_to_string(filename);

    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("Error while reading the file")),
    }
}
