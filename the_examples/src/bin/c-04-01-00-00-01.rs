// https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
//
// Varialbes are immutable by default
//

// This will error and crash
//

fn main() {
    let alfa: i32 = 123;
    alfa = 456;
}


/*
error[E0384]: cannot assign twice to immutable variable `alfa`
  --> src/bin/c-04-01-00-00-01.rs:11:5
   |
10 |     let alfa: i32 = 123;
   |         ----
   |         |
   |         first assignment to `alfa`
   |         help: consider making this binding mutable: `mut alfa`
11 |     alfa = 456;
   |     ^^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
*/

