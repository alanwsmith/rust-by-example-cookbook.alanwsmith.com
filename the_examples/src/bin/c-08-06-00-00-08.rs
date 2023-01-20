#![allow(dead_code)]
enum Widget {
    Foo,
    Bar(i32),
}

fn main() {
    let alfa = Widget::Bar(13);

    if let Widget::Bar(value @ 13) = alfa {
        println!("The value is {value}")
    }
}


// This is an example showing binding also
// works, but I don't yet know why you'd
// want to do that here
//
// This use case would be better served with:
//
//  if let Widget::Bar(13) = alfa {
//          println!("The value is 13")
//  }
//
// since you already know that explicit value. 
