// This will throw an error

mod widget {

    pub struct AlfaContainer<T> {
        contents: T,
    }

}

fn main() {

    let alfa = widget::AlfaContainer { contents: "bravo" };

}

// The reason for the error is that `contents` is
// not public (aka `pub`) like it was in the prior
// example so it's not accessible from outside
// the module. From the site "Public structs with 
// private fields cannot be constructed using field 
// names."

/* The specific error is:

error[E0616]: field `contents` of struct `AlfaContainer` is private
  --> src/bin/c-10-02-00-00-02.rs:14:39
   |
14 |     println!("alfa contains {}", alfa.contents);
   |                                       ^^^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `the_examples` due to previous error

*/

