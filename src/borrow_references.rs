fn main() {
    let s1: String = String::from("First");
    let s2: &String = &s1; //This si passign the reference
    let s3: &String = &s1;

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let mut a: String = String::from("My");

    change_value(&mut a);
    let s2: String = &mut a;

    println!("see value {}", a);
}

fn change_value(s: &mut String) {
    s.push_str(" hello friend ");
}
