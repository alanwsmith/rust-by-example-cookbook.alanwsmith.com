/* From the site

   Functions declared using `pub(super)` syntax 
   are only visible within the parent module
*/

mod widget {

    pub mod sub_widget {

        pub(super) fn alfa() {
            println!("This is widget::sub_widget::alfa")
        }

    }

    pub fn call_alfa() {
        sub_widget::alfa();
    }

}

fn main() {
    widget::call_alfa();
}

