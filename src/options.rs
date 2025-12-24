fn find_first(s: String) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }

    return None;
}

fn main() {
    let my_string: String = String::from("raman");
    let res: Option<usize> = find_first(my_string);

    match res {
        Some(index) => println!("The letter a is found in index: {}", index),
        None => println!("The letter a is not found in the string"),
    }
}
