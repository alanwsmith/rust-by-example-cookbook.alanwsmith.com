// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html
//
// Closures - Capturing 
//
// From the site:

/*
Closures are inherently flexible and will do what the 
functionality requires to make the closure work without 
annotation. This allows capturing to flexibly adapt to 
the use case, sometimes moving and sometimes borrowing. 
Closures can capture variables:

by reference: &T
by mutable reference: &mut T
by value: T

They preferentially capture variables by reference and 
only go lower when required.
*/

fn main() {
    
    let color = String::from("green");

    let alfa = || println!("The color is {color}");

    alfa();

}
