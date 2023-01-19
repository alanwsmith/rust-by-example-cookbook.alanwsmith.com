// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//
// "if let can be used to match any enum value"
//

#![allow(dead_code)]

enum Widget {
    Foo,
    Bar,
}


fn main() {
    let alfa = Widget::Foo;

    if let Widget::Foo = alfa {
        println!("alfa is Widget::Foo")
    }
}


// This is doing an equality match? I don't
// understand what let is doing here. 



