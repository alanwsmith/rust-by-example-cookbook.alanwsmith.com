/* From the site

    Functions declared using `pub(in path)` syntax are only visible
    within the given path. `path` must be a parent or ancestor module 

*/

mod widget {

    pub mod sub_widget {

        pub(in crate::widget) fn alfa() {
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

// You can't call `widget::sub_widget::alfa` in 
// `main()` with the `in create::widget` line
// in `pub` for `alfa()`

// This seems like a more advanced thing
// that you don't need to cover in the first
// run. 
//
// TODO: Find use cases for when this would 
// be helpful. 





