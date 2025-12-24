use std::fmt::Display;

struct User<'a> {
    name: &'a str,
}

fn main() {
    let first_name = String::from("jide");
    let user = User { name: &first_name }; //So this object would remain alive as far as the first_name pointer is also alive. since it a reference

    println!("The name of the user is {}", user.name);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() { x } else { y }
}
