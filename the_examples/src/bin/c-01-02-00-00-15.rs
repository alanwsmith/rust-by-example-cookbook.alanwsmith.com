fn main() {
    struct Widget(i32);
    println!("Intentional Error {}", Widget(99))
}

// Only types that implement fmt::Display can be formatted 
// with `{}`. User-defined types (like `struct Widget(i32)`) 
// do not implement fmt::Display by default


// Not sure this should go here in the examples 
// since we don't know what a struct is yet, but this 
// this just following along directly with everything
// in Rust By Example. 

// Also, when this error is shown, it should be shown how 
// to correct it. So, yeah, push this down the line a bit. 

// Here's what the error looks like: 
//
/*
error[E0277]: `Structure` doesn't implement `std::fmt::Display`
 --> src/bin/c-1-2-0-0-o.rs:3:38
  |
3 |     println!("Intentional Error {}", Structure(99))
  |                                      ^^^^^^^^^^^^^ `Structure` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Structure`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `the_examples` due to previous error
*/
