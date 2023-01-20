// https://doc.rust-lang.org/rust-by-example/variable_bindings.html
//
// This is what happens if you have an unused 
// variable without the `_` as the first character.
// 

fn main() {
    let this_will_warn: i32 = 17;
}

// You'll get this warning. 
//
//

/*
warning: unused variable: `this_will_warn`
 --> src/bin/c-04-00-00-00-03.rs:8:9
  |
8 |     let this_will_warn: i32 = 17;
  |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_this_will_warn`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `the_examples` (bin "c-04-00-00-00-03") generated 1 warning
*/

