// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

#[derive(Debug, Clone, Copy)]
// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}
impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}
// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}
// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
impl Rectangle {
    fn area(&self) -> f32 {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.bottom_right.y - self.top_left.y;
        width * height
    }

    fn rect_area(rect: &Rectangle) -> f32 {
        let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = *rect; // nested destructuring
        ((x2 - x1) * (y2 - y1)).abs()
    }

    fn square(point: &Point, side: f32) -> Rectangle {
        let bottom_right = Point { x: point.x + side, y: point.y + side };
        Rectangle { top_left: *point, bottom_right }
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let my_rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 0.0, y: 5.0 },
        bottom_right: Point { x: 5.0, y: 0.0 },
    };

    //Activity 1
    let area = Rectangle::rect_area(&my_rectangle);
    println!("The area of the rectangle is {} square pixels.\n", area);


    //Activity 2
    let another_point = Point { x: 0.0, y: 0.0 };
    let another_side = 10.0;
    let another_square = Rectangle::square(&another_point, another_side);
    let another_area = another_square.area();
    println!("The area of the square is {} square pixels.", another_area);
}