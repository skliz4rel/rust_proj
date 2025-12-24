//Ensure you dont declare an enum that is not usd in your program. Compiler wud throw an error.
enum Shape {
    Rectangle(i32, i32),
    Square(i32),
    Circle(i32),
}

fn main() {
    let rect = Shape::Rectangle(10, 20);
    let square = Shape::Square(20);
    let circle = Shape::Circle(10);

    println!("aea of the rectangle {}", cal_area(rect));

    println!("area of the square {}", cal_area(square));
}

fn cal_area(shape: Shape) -> i32 {
    return match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
        Shape::Circle(side) => 3 * side,
    };
}
