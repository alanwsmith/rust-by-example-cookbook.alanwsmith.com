// https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
//
// From the page: 
// 
/*

loop
Rust provides a loop keyword to indicate an infinite loop.

The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one.
*/

fn main() {
    let mut count: i32 = 0;

    loop {
        count += 1;
        println!("The value is {}", count);

        if count == 5 {
            break;
        }
    }
}
