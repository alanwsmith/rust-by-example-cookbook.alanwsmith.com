// https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
//
// If you assign something to a mutalbe variable 
// then change it without using it, you'll get
// a warning. 
//

fn main() {
    let mut alfa: i32 = 123;
    alfa = 456;
    println!("The value is {}", alfa)
}

// An example of the error is:

/*
warning: value assigned to `alfa` is never read
 --> src/bin/c-04-01-00-00-03.rs:9:13
  |
9 |     let mut alfa: i32 = 123;
  |             ^^^^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?
*/

 

