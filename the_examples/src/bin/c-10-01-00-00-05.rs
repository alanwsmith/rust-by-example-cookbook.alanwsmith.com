/* From the site

    Functions declared using `pub(self)` syntax are only visible within
    the current module, which is the same as leaving them private

*/

mod widget {

    pub mod sub_widget {

        pub(self) fn alfa() {
            println!("This is widget::sub_widget::alfa")
        }

        pub fn call_alfa() {
            alfa();
        }

    }

}

fn main() {
    widget::sub_widget::call_alfa();
}


// Calling widget::sub_widget::alfa() would 
// throw an error. 
//
// TODO: Find use cases for this. 
//
