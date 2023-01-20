// https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
//
// // From the page
//

/*
Branching with if-else is similar to other languages. 
Unlike many of them, the boolean condition doesn't 
need to be surrounded by parentheses, and each 
condition is followed by a block. if-else conditionals 
are expressions, and, all branches must return the same type.
*/

fn main() {

    let alfa = 123;

    if alfa < 0 {
        println!("{} is negative", alfa);
    } else if alfa > 0 {
        println!("{} is positive", alfa);
    } else {
        println!("{} is zero", alfa);
    }

}


// TODO: Run one of these that just shows the if/else 
// first and then to if/else if/else as it's own thing
//
