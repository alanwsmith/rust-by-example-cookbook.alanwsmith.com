// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
//

/*
    Struct visibility
    Structs have an extra level of visibility with their fields. 
    The visibility defaults to private, and can be overridden 
    with the pub modifier. This visibility only matters when a 
    struct is accessed from outside the module where it is 
    defined, and has the goal of hiding information (encapsulation).
*/

mod widget {

    pub struct AlfaContainer<T> {
        pub contents: T,
    }

}

fn main() {
    let alfa = widget::AlfaContainer { contents: "bravo" };

    println!("alfa contains {}", alfa.contents);

}

// This shows how you can make a new thing by
// passing in the data for the public field. 
