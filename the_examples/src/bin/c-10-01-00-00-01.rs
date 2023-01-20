// https://doc.rust-lang.org/rust-by-example/mod/visibility.html
//
// Module Visibility
//

/* From the site:

   By default, the items in a module have private 
   visibility, but this can be overridden with the 
   pub modifier. Only the public items of a module 
   can be accessed from outside the module scope.

*/

mod widget {
    pub fn alfa() {
        println!("This is widget::alfa")
    }
}

fn main() {
    widget::alfa();
}
