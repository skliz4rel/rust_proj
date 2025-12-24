fn main() {
    let word = "mad man"; //Thisis a String slice or string literal

    let mut name: String = String::from("Jide");
    name.push_str(" akindejoye.");

    name.replace_range(8..name.len(), "");

    println!("name is {}", name);

    let sentence: String = String::from("America This is the best day of my life");
    let first = first_word(&sentence);

    println!("This is the first word {}", first);

    //A more optimized method
    let first = find_first_word(&sentence);

    println!("This is the first word {}", first);

    slice_string();
}

fn first_word(sentence: &String) -> String {
    let mut first: String = String::new();

    for s in sentence.chars() {
        if s == ' ' {
            break;
        }

        first.push_str(&s.to_string());
    }

    return first;
}

fn slice_string() {
    let word = String::from("Felix This is what I want");
    let word2 = &word[0..5];

    println!("{}", word2);
}

fn find_first_word(text: &String) -> &str {
    let mut space_index: usize = 0;
    for (_, s) in text.chars().enumerate() {
        if s == ' ' {
            break;
        }

        space_index = space_index + 1;
    }

    return &text[0..space_index];
}
