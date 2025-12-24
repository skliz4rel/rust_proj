//Lifetimes: It is used to solve the problem of a danglin pointer. A scenario were the pointer is referenceing an object that has been deallocated from the Heap memory
//Introduce a generi clifetime annotation

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}
