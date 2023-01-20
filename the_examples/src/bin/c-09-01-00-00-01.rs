// https://doc.rust-lang.org/rust-by-example/fn/methods.html
//
// Associated functions & Methods
// 
// From the site: "Some functions are connected to a 
// particular type. These come in two forms: associated 
// functions, and methods. Associated functions are 
// functions that are defined on a type generally, while 
// methods are associated functions that are called on a 
// particular instance of a type.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(new_x: f64, new_y: f64) -> Point {
        Point { x: new_x, y: new_y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {

    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }

}



fn main() {
    let alfa = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", alfa.perimeter());
    println!("Rectangle area: {}", alfa.area());


    // TODO: Split this mutalbe example
    // to it's own thing
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);


}


// The `translate` method has `&mut` because it needs
// to be mutable. 
//
