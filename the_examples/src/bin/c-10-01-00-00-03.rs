// Modules can also be nested. 
//

mod widget {

    pub mod sub_widget {

        pub fn alfa() {
            println!("This is widget::sub_widget::alfa")
        }

    }

}

fn main() {
    widget::sub_widget::alfa();
}


