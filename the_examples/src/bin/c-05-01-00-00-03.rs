// https://doc.rust-lang.org/rust-by-example/types/cast.html
//
// Not everything can be converted into every other
// format. For example, you can't convert floats
// into chars. 
// 
// This will be a compiler error 


fn main() {

    let alfa: f32 = 12.345;

    let bravo: char = alfa as char;

}


// ERROR:

/*
error[E0604]: only `u8` can be cast as `char`, not `f32`
  --> src/bin/c-05-01-00-00-03.rs:14:23
   |
14 |     let bravo: char = alfa as char;
   |                       ^^^^^^^^^^^^ invalid cast
   |
help: try `char::from_u32` instead (via a `u32`)
  --> src/bin/c-05-01-00-00-03.rs:14:23
   |
14 |     let bravo: char = alfa as char;
   |                       ^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0604`.
*/

