// This won't print anything because
// alfa is Widget::Foo and not
// Widget::Bar
//

#![allow(dead_code)]
enum Widget {
    Foo,
    Bar,
}

fn main() {
    let alfa = Widget::Foo;

    if let Widget::Bar= alfa {
        println!("alfa is Widget::Foo")
    }
}


