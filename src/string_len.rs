fn main() {
    let value = String::from("This is the string lenght");

    let len = get_string_len(&value);

    println!("This is the leng of the string {}", len);
}

fn get_string_len(s: &String) -> usize {
    // s.chars().count() //same has statement below
    return s.chars().count();
}
