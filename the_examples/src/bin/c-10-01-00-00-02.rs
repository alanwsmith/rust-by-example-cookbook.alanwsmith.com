// https://doc.rust-lang.org/rust-by-example/mod/visibility.html
//
// You can access private functions in a module
// through public functions.  


mod widget {

    fn alfa() {
        println!("This is private widget::alfa");
    }

    pub fn bravo() {
        println!("This is public widget::bravo");
        alfa();
    }

}

fn main() {
    widget::bravo();
}



