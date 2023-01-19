// https://doc.rust-lang.org/rust-by-example/flow_control/while.html
//

/*
while

The while keyword can be used to run a loop while a 
condition is true.
*/

fn main() {
    let mut counter: i32 = 0;

    while counter <= 5 {
        println!("The value is {}", counter);
        counter += 1;
    }
}


