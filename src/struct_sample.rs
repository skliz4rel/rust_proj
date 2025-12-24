struct Person; //struct with no attributes

impl Person {
    fn display(&self) -> String {
        return String::from("This is a person object ");
    }
}

struct User {
    name: String,
    age: u32,
    active: bool,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

//generic enums
struct Point<A, B> {
    x: A,
    y: B,
    z: B,
}

fn main() {
    let integer_points = Point {
        x: 1,
        y: "hello",
        z: "world",
    };

    println!(
        "Integer Point: ({}, {})",
        integer_points.x, integer_points.y
    );

    let person: Person = Person;
    println!("Struct with no attribute sample {}", person.display());

    let name: String = String::from("Alice");
    let user: User = User {
        name: name,
        age: 30,
        active: true,
    };

    println!("This is the details of the user {} {}", user.name, user.age);

    let rect: Rect = Rect {
        width: 30,
        height: 40,
    };

    println!(
        "This is the area {} and this is the perimeter {}",
        rect.area(),
        rect.perimeter()
    );
}
