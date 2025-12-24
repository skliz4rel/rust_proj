fn main() {
    let x: i32 = 10; //created on the stack, obwer is the main function
    let y: i32 = 20; //created on the stack, obwer is the main function

    let result: i32 = y + x; ////created on the stack, obwer is the main function

    println!("The sum of x {} and y {} is equal result {}", x, y, result);
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b; //owner of a b c is the sum function
    println!("The sum of two numbers a {} and b{} is result {}", a, b, c);

    return c;
}

fn next() {
    let mut x = 1; //created on the stack
    {
        let y = 10; //created on stack and owned with the block

        x += y;
    }

    println!("This is the value of y {}", x);
}

//This is mostly the error prone part
fn heapVariables() {
    let s1: String = String::from("Jide Akidnejoye is my boyfriend");
    let s2: String = s1; //s2 is now the owner of this "Jide Akidnejoye is my boyfriend"

    println!("This is the value in the string {}", s2);
}
