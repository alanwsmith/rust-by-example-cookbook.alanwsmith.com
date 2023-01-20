// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// You can't use the debug priting for tuples more
// than 12 elements. This throws an error


fn main() {
    let to_long_tuple = (
        1, 2, 3, 4, 5, 6, 7, 8, 
        9, 10, 11, 12, 13
    );
    println!("{:?}", to_long_tuple)
}


// The error is: 

/*
error[E0277]: `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})` doesn't implement `Debug`
  --> src/bin/c-02-02-00-00-05.rs:12:22
   |
12 |     println!("{:?}", to_long_tuple)
   |                      ^^^^^^^^^^^^^ `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `({integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer}, {integer})`
   = help: the following other types implement trait `Debug`:
             ()
             (A, Z, Y, X, W, V, U, T)
             (B, A, Z, Y, X, W, V, U, T)
             (C, B, A, Z, Y, X, W, V, U, T)
             (D, C, B, A, Z, Y, X, W, V, U, T)
             (E, D, C, B, A, Z, Y, X, W, V, U, T)
             (T,)
             (U, T)
           and 5 others
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `the_examples` due to previous error
*/
