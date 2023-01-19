// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
//
// A Pretty printed example using `{:#?}` instead
// of `{:?}`.

#[derive(Debug)]
struct Widget(i32);

#[derive(Debug)]
struct Wrapper(Widget);

fn main() {
    println!(
        "Wrappped Widget {:#?}",
        Wrapper(Widget(9191))
    )
}


// OUTPUTS:
// Wrappped Widget Wrapper(
//   Widget(
//     9191,
//   ),
// )



