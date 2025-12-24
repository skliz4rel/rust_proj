//Traits are similar to asbtract class in other languages like Java, although with some differences
//A trait defines the functionality of a particular type has and can share with other types. We can use traits to define the shared behaivour in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
pub trait Summary {
    fn displayName(&self) -> String; //This method or function has not be defined. so you do it in the struct implementation

    fn summarize(&self) -> String {
        return String::from("Summarize");
    }
}

pub trait Person {
    fn university(&self) {
        println!("The person university");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn displayName(&self) -> String {
        return format!("My name is {} and my age is {}", self.name, self.age);
    }

    //Overriding the defualt function in the trait
    fn summarize(&self) -> String {
        return String::from("Override Summarize");
    }
}

//Adding another Trait for the user struct
impl Person for User {}

struct Fix;
impl Summary for Fix {
    fn displayName(&self) -> String {
        String::from("Diplaying the name for Fix Struct")
    }
}
impl Summary for String {
    fn displayName(&self) -> String {
        String::from("Diplaying the name for String Object")
    }
}

fn main() {
    let user = User {
        name: String::from("Jide Akindejoye"),
        age: 21,
    };

    println!("{}", user.summarize());

    let f = Fix;
    notify(&f);

    notify(&String::from("Jide Triats for String"));

    notify_generics(&f);

    notify_generics_multiple(&user);
}

fn notify(u: &impl Summary) {
    println!("{}", u.summarize());
}

pub fn notify_generics<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//This method should show passing object referencing multiple traits
pub fn notify_generics_multiple<T: Summary + Person>(item: &T) {
    println!(
        "Breaking news! shows inheriting multiple traits {}",
        item.summarize()
    );
}
