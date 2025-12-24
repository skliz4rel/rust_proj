fn main() {
    let num: i32 = 4;
    println!("Fibonacci of {} is equal to {}", num, fib(2));
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return 1;
    }

    // 0 1 1 2 3 5

    for i in 1..(num - 1) {
        let temp = second; // 1
        second = second + first; // 1
        first = temp;
    }

    return second;
}
