use std::thread;
use std::time::Duration;

use std::sync::mpsc;

fn main() {
    multiple_tranmitter_reciever();

    thread_with_variable();

    channel();

    //Thread 1
    thread::spawn(|| {
        for i in 1..10 {
            println!("Fuck {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    //THread 2
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    //handle.join().unwrap(); //This tells the thread belwo to wait

    //main thread
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_with_variable() {
    let x = 1;
    {
        let v = vec![1, 2, 3];
        thread::spawn(move || {
            //The move keyword. moves ownership of the vector v to thread.
            println!("{:?}", v);
        });
    }
    //You wont be able to use v outside thread.

    println!("{}", x);
}

/*
Messaging passing is when threads pass data from one thread to another
powered through a utility called channel in Rust.
A channel has 2 halves a transmitter and a receiver. The transmitter half is the upstream location were you put the rubber ducks int the river.
One part of your code calls the methods on the transmitter with the data that you want to send. and the second part checks the receiving end for arriving messages. A channel is said to be closed it either the transfmitter and he receiver half is dropped.
 */

fn channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi sexy");
        tx.send(val).unwrap(); //do not unwrap, it crashes your program
    });

    let received = rx.recv(); //.unwrap();

    match received {
        Ok(value) => println!("Got: {value}"),
        Err(_err) => println!("Error while reading the data from the thread transmitter"),
    }
}

//Using mutiple transmitter to solve a complex problem. Breaking into multiple threads and transmitting result to one reciever

fn multiple_tranmitter_reciever() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();

        thread::spawn(move || {
            //move allows the producer obj to be accessible inside the thread
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx); //This drops the first transmitter that was cloned 

    let mut ans: u64 = 0;
    //We iterate through the various reciever since multiple tran
    for val in rx {
        ans = ans + val;
        println!("found value");
    }

    println!("Ans is {}", ans);
}
