// https://doc.rust-lang.org/rust-by-example/mod/use.html

/*
    The use declaration

    The use declaration can be used to bind a full path 
    to a new name, for easier access.
*/

use alfa::bravo::{
    charlie
};

mod alfa {
    pub mod bravo {
        pub fn charlie() {
            println!("This is alfa::bravo::charlie");
        }
    }
}

fn main() {

    charlie();

}

// This lets you use `charlie()` instead of
// the full path `alfa::bravo::charlie()`


