#![allow(dead_code)]
enum Widget {
    Foo,
    Bar(i32),
}

fn main() {
    let alfa = Widget::Bar(18);

    if let Widget::Bar(value) = alfa {
        println!("The value is {value}")
    }
}



