// https://doc.rust-lang.org/rust-by-example/flow_control/while.html
//
// This is an example with a starting
// condition that's false so the while
// loop is never executed. 

fn main() {
    let mut counter = 10; 

    while counter < 5 {
        println!("The updated value is {counter}");
        counter += 1;
    }

    println!("The final value is {counter}")
}

// Write up how you'll never see the
// internal println since the counter is
// already above the comparison value 
// before the loop starts



