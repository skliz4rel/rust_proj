enum Direction {
    North,
    East,
    South,
    West,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    let my_direction: Direction = Direction::North;
    move_arround(my_direction);

    let circle: Shape = Shape::Circle(10.0);
    let square: Shape = Shape::Square(20.0);
    let rect: Shape = Shape::Rectangle(10.0, 20.0);

    let area = calculate_area(circle);
    println!("This is the area of the cricle {}", area);

    let area = calculate_area(square);
    println!("This is the area of the square {}", area);

    let area = calculate_area(rect);
    println!("This is the area of the rect {}", area);
}

fn move_arround(d: Direction) {}

fn calculate_area(shape: Shape) -> f64 {
    let answer: f64 = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_len) => side_len * side_len,
        Shape::Rectangle(width, height) => width * height,
    };

    return answer;
}
