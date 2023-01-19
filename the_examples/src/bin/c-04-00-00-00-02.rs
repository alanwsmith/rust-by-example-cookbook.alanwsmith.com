// https://doc.rust-lang.org/rust-by-example/variable_bindings.html
//
// This is how to prevent the compiler from
// warning you about an unuse variable
//

// So this will throw a warning
// fn main() {
//     let unuse_var: i32 = 15;
// }


// and this is okay
fn main() {
    let _unuse_var: i32 = 15;
}


