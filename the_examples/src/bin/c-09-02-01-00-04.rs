// This code is an expected failure. 
//

fn main() {

    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("Count is {count}");
    };

    increment();

    let _this_causes_an_error = &count;

    increment();

}


// The error message is

/*
error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
  --> src/bin/c-09-02-01-00-04.rs:12:21
   |
5  |     let mut increment = || {
   |                         -- mutable borrow occurs here
6  |         count += 1;
   |         ----- first borrow occurs due to use of `count` in closure
...
12 |     let _reborrow = &count;
   |                     ^^^^^^ immutable borrow occurs here
13 |
14 |     increment();
   |     --------- mutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `the_examples` due to previous error
*/

// TODO: Figure out more about how to 
// explain what's happening here. 
