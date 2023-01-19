// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Using on struct as a field for another struct. 

struct Point {
    x: i32,
    y: i32
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    let start_corner = Point { x: 12, y: 24 };
    let end_corner = Point { x: 37, y: 41 };

    let the_box = Rectangle { 
        top_left: start_corner,
        bottom_right: end_corner
    };

    println!(
        "Values {} {} {} {}",
        the_box.top_left.x,
        the_box.top_left.y,
        the_box.bottom_right.x,
        the_box.bottom_right.y
    )
}


// Note, you can't use box for a varible name apparently. 
//
// OUTPUT
// Values 12 24 37 41
//
