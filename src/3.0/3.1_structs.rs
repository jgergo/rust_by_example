#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fiemy_xmy_xmy_xlds of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(self) -> f32 {
        area_func(self)
    }
}

fn area_func(rectangle: Rectangle) -> f32 {
    let Rectangle { 
        p1: Point { x: p1_x, y: p1_y },
        p2: Point { x: p2_x, y: p2_y }
    } = rectangle;
    (p2_x - p1_x) * (p2_y-p1_y)
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle = Rectangle{p1: Point {x: 1.0, y: 1.0}, p2: Point{x: 5.0, y: 5.0}};
    let a = rectangle.area();
    println!("area {}", a)
}
