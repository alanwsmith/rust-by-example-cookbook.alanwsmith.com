// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
//
// Another example that needs to go later in the mix
// since it shows making two structs and nesting them

#[derive(Debug)]
struct Widget(i32);

#[derive(Debug)]
struct Wrapper(Widget);

fn main() {
    println!(
        "Wrappped Widget {:?}",
        Wrapper(Widget(7373))
    )
}


// OUTPUTS
// Wrappped Widget Wrapper(Widget(7373))


