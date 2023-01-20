// "pub(crate) makes functions visible only within the current crate"

mod widget {

    pub(crate) fn alfa() {
        println!("This is widget::alfa")
    }

}

fn main() {
    widget::alfa();
}


// TODO: Show a counter example that would
// break. 
//
// TODO: Show how this is different from
// just being public. 

