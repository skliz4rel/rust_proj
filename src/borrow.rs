fn main() {
    let mut s1: String = String::from("This is my string");

    do_something(&mut s1);

    println!("s1 {}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" femi ");
    println!("Valeu is sent from parent {}", s2);
}
