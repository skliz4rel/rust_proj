fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // let vec = vec![1, 2, 3]; //This approach when you konw all elements to initialize

    println!("Original vector: {:?}", vec);

    let even_vec = even_filter(&vec);
    println!("Even vector: {:?}", even_vec);

    // original vector remains unchanged
    println!("Original vector again: {:?}", vec);
}

fn my_vectors() {
    let vectors: Vec<i32> = vec![1, 2, 3, 4];

    for num in vectors {
        println!("{}", num);
    }
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val); // dereference i32
        }
    }

    new_vec
}
