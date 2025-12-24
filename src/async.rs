use tokio;

#[tokio::main]
async fn main() {
    caller().await;
}
async fn read_from_database() -> String {
    "DB Result".to_owned()
}

async fn caller() {
    println!("I am an async function caller");
    let s1: String = read_from_database().await;

    println!("FIrst result: {s1}");

    let s2: String = read_from_database().await;

    println!("FIrst result: {s2}");
}
