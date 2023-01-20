// You can add an `as` to the `use` to use a 
// different name

use alfa::bravo::charlie as delta;

mod alfa {
    pub mod bravo {
        pub fn charlie() {
            println!("This is alfa::bravo::charlie");
        }
    }
}

fn main() {

    delta();

}

