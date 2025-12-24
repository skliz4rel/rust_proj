fn main() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8]; //array list

    let list_slice = &list[0..4];

    println!("{} \n {}", list, list_slice);
}
