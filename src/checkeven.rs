fn main() {
    let number: i32 = 29;

    println!("is even {}", check_even(number));
}

pub fn is_even(number: i32) {
    if number % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn check_even(number: i32) {
    if number % 2 == 0 {
        println!("{} is an even number", number);
    } else {
        println!("{} is an odd number", number);
    }
}
