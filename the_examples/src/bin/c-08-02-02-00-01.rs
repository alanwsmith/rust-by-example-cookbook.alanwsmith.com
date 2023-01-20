// https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html
//

/*
Returning from loops
One of the uses of a loop is to retry an operation 
until it succeeds. If the operation returns a value 
though, you might need to pass it to the rest of the 
code: put it after the break, and it will be returned 
by the loop expression.
*/

fn main() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The value is {}", counter)

}


