// https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
//
// You can prevent the warning with:
// #[allow(unused_assignments)]

#[allow(unused_assignments)]
fn main() {
    let mut alfa: i32 = 123;
    alfa = 456;
    println!("The value is {}", alfa)
}
