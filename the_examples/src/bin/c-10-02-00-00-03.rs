// "However, structs with private fields can be 
// created using public constructors"

mod widget {

    pub struct AlfaContainer<T> {
        contents: T,
    }

    impl<T> AlfaContainer<T> {
        pub fn new(contents: T) -> AlfaContainer<T> {
            AlfaContainer {
                contents: contents,
            }
        }
    }

}

fn main() {

    let alfa = widget::AlfaContainer::new("bravo");

    // println!("alfa contains {}", alfa.contents);

}

// This compiles without the `println!` that's 
// commented out in `main()`. If I uncomment that, 
// I get this error: 

/*

   error[E0616]: field `contents` of struct `AlfaContainer` is private
  --> src/bin/c-10-02-00-00-03.rs:24:39
   |
24 |     println!("alfa contains {}", alfa.contents);
   |                                       ^^^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `the_examples` due to previous error


*/

// That expected since the field is private. What this
// shows is that you can make the thing and would be
// able to use other functions and fields if they
// were private. 


