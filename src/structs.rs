//THe   let you structure data
//struct are like classes
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }

    fn increaseWidth(&self, newWidth: i32) -> i32 {
        self.width + newWidth
    }

    fn staticFunction() -> String {
        return String::from("This is a rectangle Struct");
    }
}

fn main() {
    let user = User {
        first_name: String::from("Jide"),
        last_name: String::from("Akindejoye"),
        age: 32,
    };

    println!("{}", user.first_name);

    let rectangle = Rect {
        width: 20,
        height: 10,
    };

    println!("This is the area of the rectangle {}", rectangle.area());
    println!("This is the perimeter {}", rectangle.perimeter());
    println!(
        "New width for the rectangle is {}",
        rectangle.increaseWidth(10)
    );
    println!(
        "A static function for rectangle struct {}",
        Rect::staticFunction()
    );
}
