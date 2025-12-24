use std::collections::HashMap;
//for vectors , hashmaps, list use iterators when you loop tru them

fn main() {
    immutable_iterator();

    mutable_iterator();

    iterator_while_loop();

    into_iterator();

    consumer_methods();

    adaptor_methods();

    practice();

    iterator_with_hashmap();
}

fn immutable_iterator() {
    let nums = vec![1, 2, 3, 4];

    let iterator = nums.iter(); //iterator borrows the values inthe vector

    for i in iterator {
        println!("{}", i);
    }

    println!("{:?}", nums);
}

fn mutable_iterator() {
    let mut nums = vec![1, 2, 3, 4];

    let iterator = nums.iter_mut(); //iterator borrows the values inthe vector

    for val in iterator {
        *val = *val + 1; //this allows you to update the values while you iterate tru it.
    }

    println!("{:?}", nums);
}

fn iterator_while_loop() {
    let nums = vec![1, 2, 3, 4];
    let mut iter = nums.iter();

    while let Some(val) = iter.next() {
        print!("{}", val);
    }
}

//This interator type takes ownership of the collection. It is memory safer that other iterator
//only use this when you dont need the collection afer itertaing thro it.
//This is the default iterator used in collections
fn into_iterator() {
    let nums = vec![1, 2, 3];

    let iter = nums.into_iter();

    for value in iter {
        print!("{} ", value);
    }

    // println!("{:?}", nums); ownership makes the collection go out of reference.
}

fn consumer_methods() {
    //There are 2 types, THe consumer methods that consume the iterator so it cant be used again
    //The second method is the iterator adaptor methods, The modify iterator but they dont consume it.

    println!("This is a display of the consumer method !");

    let sum = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = sum.iter();
    let total: i32 = iter.sum();

    println!(" Summation is {}", total);

    // iter.next(); //This iterator has been consumed so it out of scope
}

//This method would create a modified copy of the iterator
fn adaptor_methods() {
    println!("This is the display of the adapter method --->");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = nums.iter();
    let iter2 = iter.map(|x| x + 1);

    for x in iter2 {
        print!("{} ", x);
    }

    //Let try the filter adapter method
    println!("\nUsing the filter method in the iterator !!");
    let iter1 = nums.iter();
    let iter3 = iter1.filter(|x| *x % 2 == 0);
    for i in iter3 {
        print!("{} ", i);
    }
}

fn practice() {
    println!("Practice Sample code\nExtract old number and double each item in the list");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let iter = numbers.iter();

    let iter1 = iter.filter(|x| *x % 2 != 0);

    let iter2 = iter1.map(|x| x * 2);

    let new_vec: Vec<i32> = iter2.collect();

    println!("{:?}", new_vec);
}

//THis function is goign to show the use of iterators with hash map
fn iterator_with_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("jide"), 37);
    scores.insert(String::from("gbemi"), 32);
    scores.insert(String::from("bolaji"), 40);

    let iterator = scores.iter();

    for (key, value) in iterator {
        println!("Key={} Value={}", key, value);
    }
}
