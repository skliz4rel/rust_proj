use std::collections::HashMap;

fn main() {
    println!("A practive rust code ");

    primitive_datatypes();

    compound_datatypes();

    human_id("Jide", 20, 1.8);

    fun_expression();

    println!("Add two number {}", add(1, 2));

    println!("My BMI is {}", bmi(1.5));

    ownership();

    var_and_immutablity();

    constants();

    shadowing();

    conditions();

    loops();

    structs();

    option_enums();

    collections();
}

fn primitive_datatypes() {
    //char, int, float, bool

    let a: i16 = -4;
    let b: i8 = -5;
    let num: i32 = -20;
    let num1: u32 = 10023;
    let num2: u64 = 2452534523523424;
    let num3: u128 = 34523453452345324523454;

    //Float datatypes
    let p: f32 = 1.0;
    let pi: f64 = 3.142;

    //Boolean values
    let check: bool = true;
    let check1: bool = false;

    //Charater type
    let letter: char = 'a';
}

//compound datatypes.

fn compound_datatypes() {
    //arrays, tuples, slices, and strings (string slices)

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    let fruits: [&str; 3] = ["fruit", "Banana", "grape"];
    println!("Println the first element in the fruit {}", fruits[0]);

    //Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("{:?}", human);

    let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Jide", 23, true, [1, 2, 3, 4, 5]);
    println!("{:?}", my_mix_tuple);

    //Slices []
    let number_slices: &[i32] = &[1, 2, 3, 4, 5]; //This is a number slice.
    println!("{:?}", number_slices);

    let animal_slices: &[&str] = &["liion", "elephant", "crocodile"];
    println!("{:?}", animal_slices);

    let book_slices: &[String] = &[
        String::from("God NOt be blamed"),
        String::from("Demon Forest"),
    ]; //String slices

    println!("{:?}", book_slices);

    //String vs String slices. String are owned not borrowed. String slice is a reference with fix size

    //Strings
    let mut stone_cold: String = String::from("From helo, ");
    stone_cold.push_str(" friend");
    println!("{}", stone_cold);

    //string slices-----Good for memory efficiency. used when you dont want to take ownership of the string data
    let string: String = String::from("hello world");
    let str_slice: &str = &string[0..5];
    println!("printing the string slice {}", str_slice);
}

//functions
fn test_function(mut name: String) -> String {
    name.push_str(" My test function");
    name
    //or
    // return name;
}

fn tell_height(height: u32) {
    println!("My height is {} cm ", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm",
        name, age, height
    );
}

fn fun_expression() {
    let x: i32 = {
        let price: i32 = 30;
        let qty: i32 = 2;
        price * qty //or return price* qty;  it same thing
    };

    println!("This is the result {}", x);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bmi(height: f32) -> f32 {
    (height / 1000.0) * (height * 2.0)
}

//ownership, borrowing, references
fn ownership() {
    let str: String = String::from("This is the name of the lord");
    println!("This is the lenght of the string {}", calculate_len(str));
}

fn calculate_len(s: String) -> usize {
    s.len()
}

fn references() {
    let mut x: i32 = 10;
    let y: &mut i32 = &mut x;

    *y += 5; //deerefencing

    println!("y = {}", y);
}

struct Bank {
    owner: String,
    balance: f64,
}

impl Bank {
    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance {}",
            self.owner, self.balance
        );
    }
}

fn useBank() {
    let mut account: Bank = Bank {
        owner: String::from("Olajide Akindejoye"),
        balance: 1000.0,
    };

    account.check_balance();

    //withdraw money
    account.withdraw(50.0);

    account.check_balance();
}

fn var_and_immutablity() {
    let mut a: u16 = 5;

    a = 10;

    println!("The value of a {}", a);
}

fn constants() {
    const Y: i32 = 10;

    println!("Ths is the constact {}", Y);
}

fn shadowing() {
    let a: i32 = 10;
    let a: i32 = 20;

    {
        let a = a + 30;
        println!("From inner value {}", a);
    }

    println!("This is shawdoing {}", a);

    //U can store different datatypes in the variable
    let a: String = String::from("Now a is storing a string thanks to Shadowing");
    println!("A as a String variable {}", a);
}

fn conditions() {
    let a: i32 = 10;

    if a > 0 {
        println!("This a is greater than 0");
    } else if a == 10 {
        println!("A is equal to 10");
    } else if a > 10 {
        println!("A is greater than 10");
    } else {
        println!("A is less than 10");
    }

    let condition: bool = true;
    let number: i32 = if condition { 6 } else { 5 };

    println!("Value of number {}", number);
}

fn loops() {
    loop {
        println!("Hello world from Loop");
        break;
    }

    for i in 1..100 {
        print!("{}, ", i);
    }

    let mut counter: i32 = 0;

    counter = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("\nCounter  = {}", counter);

    let array: [i32; 4] = [1, 2, 3, 4];
    for y in array {
        println!("element in the array {y}");
    }
}

struct User {
    username: String,
    active: bool,
    password: String,
    email: String,
    signin_account: u64,
}

fn structs() -> User {
    //tuple
    let rect: (i32, i32) = (10, 10);

    //Struct

    struct Book {
        title: String,
        author: String,
        pages: u16,
        available: bool,
    }

    let mut user = User {
        username: String::from("skliz"),
        active: false,
        password: String::from("password"),
        email: String::from("skliz4rel@gmail.com"),
        signin_account: 1,
    };

    user.email = String::from("skliz4rel@yahoo.com");

    println!("User email is {}", user.email);

    return user;
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let white: Color = Color(255, 255, 255);

    //unitlike struct or empty struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
}

enum IPAddKind {
    V4,
    V6,
}

fn enums() {
    let four = IPAddKind::V4;
    let six = IPAddKind::V6;
}

struct IpAddress {
    kind: IPAddKind,
    address: String,
}

fn route() {
    let home: IpAddress = IpAddress {
        kind: IPAddKind::V4,
        address: "124.832.234.234".to_string(),
    };

    let loopback = IpAddress {
        kind: IPAddKind::V6,
        address: "124.832.234.234".to_string(),
    };
}

fn enums_with_data() {
    enum IP {
        IPV4(u32, u32, u32, u32),
        IPV6(u32, u32, u32, u32),
    };

    let IPV4 = IP::IPV4(124, 12, 34, 24);
    let IPV6 = IP::IPV6(124, 12, 34, 24);
}

fn option_enums() {
    let result: Option<f64> = divideOption(10.0, 5.0);

    match result {
        Some(value) => println!("Division result is {}", value),
        None => println!("The denomication was 0"),
    }

    let result1: Result<f64, String> = divideResult(10.0, 5.0);

    match result1 {
        Ok(value) => println!("Division result is {}", value),
        Err(err) => println!("The denomication was 0 {}", err),
    }
}

fn divideOption(numerator: f64, denomenator: f64) -> Option<f64> {
    if denomenator == 0.0 {
        return None;
    } else {
        return Some(numerator / denomenator);
    }
}

fn divideResult(numerator: f64, denomenator: f64) -> Result<f64, String> {
    if denomenator == 0.0 {
        return Err("Cannot divide by 0".to_string());
    } else {
        return Ok(numerator / denomenator);
    }
}

fn collections() {
    vectors();

    utf8_str();

    hashmaps();
}

fn hashmaps() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("jide"), 30);
    scores.insert(String::from("gbemi"), 26);

    for (key, value) in &scores {
        println!("key {} and value {}", key, value);
    }

    let key: &str = "jide";

    let score = scores.get(key).copied().unwrap_or(0);

    println!("Jide score {}", score);
}

fn utf8_str() {
    let s1: String = String::from("Jide ");
    let s2: String = String::from("Akindejoye");
    let s3: String = s1 + &s2; //using the plust sign. The second string must be a reference , cos we took ownership of the first one. So we cant take ownership of two variables at a go.

    println!("This is value of string {}", s3);

    let sch = "in baptist";
    let s3: String = format!("{s3} {sch}");
    println!("{}", s3);
}

fn vectors() {
    //vector
    let mut v: Vec<String> = Vec::new();
    v.push("j".to_string());
    v.push("c".to_string());

    let mut v1: Vec<i32> = vec![1, 2, 3, 4];
    v1.push(5);

    println!("v1 {:?} and v2 {:?} ", v, v1);

    let first: &i32 = &v1[0];
    let third: &i32 = &v1.get(2).unwrap(); //second approach. This unrap let the program to fail or panic if not found

    //reading the elevment of a vector
    println!("Get elements in vector first {} and third {}", first, third);
}
