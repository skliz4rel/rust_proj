fn main() {
    println!("Hello, world! for beginers ");

    let a: i32 = 10;
    let b: i32 = 10;
    let result: f64 = 0.0;

    let check: bool = false;

    let name: String = String::from("Akindejouye Olajide");

    let character: Option<char> = name.chars().nth(0);

    match character {
        Some(ch) => println!("character found {}", ch),
        None => println!("There is no data found in that position"),
    };

    println!("pringint a letter in my name {}", name);

    //result = a + b;

    println!("x {} b {} result {}", a, b, result);
    println!("This is the check {}", check);

    if check {
        println!("This value is true");
    } else {
        println!("This value is false   ");
    }

    //conditions , loops and functions

    let ismale: bool = true;

    if ismale {
        println!("is a male");
    } else {
        println!("is a femlae");
    }

    //Loop
    for i in 0..10 {
        print!("This is the value in the index {}", i);
    }

    let sentence: String = String::from("This is the frist sentence");
    let result = first(sentence);

    println!("This is the first word in the senstence {}", result);
}

fn first(s: String) -> String {
    let mut newStr: String = String::new();

    for char in s.chars() {
        newStr.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return newStr;
}

//Mutability
fn do_mutability() {
    let a: u32 = 19; //This is immutable. all rust variables are immutable by default

    let mut name: String = String::from("Jide Akindejoye");
}
