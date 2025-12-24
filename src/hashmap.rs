use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("jide"), 22);
    users.insert(String::from("raman"), 32);

    let first_user_age = users.get("jide");

    match first_user_age {
        Some(value) => println!("{}", value),
        None => println!("No value was found "),
    }

    //This function would call a vector with tuple
    vector_with_tuple();
}

//Vecto with a Tuple that returns a HashMap
//A Tuple is a datastrcuture that has two pairs

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn vector_with_tuple() {
    let input_vec = vec![(String::from("jide"), 22), (String::from("femi"), 32)];

    let hm = group_values_by_keys(input_vec);

    println!("{:?}", hm);
}
