// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//
// This is how you can specifcy an error 
// case if the destructure dones't work

// TODO: Look at this for more details on how if let works
// https://doc.rust-lang.org/book/ch06-03-if-let.html

fn main() {

    let alfa: Option<i32> = None;

    if let Some(bravo) = alfa {
        println!("Matched {bravo}")
    } else {
        println!("No match")
    }

}

// Guessing None type doesn't destructure
// here?


