// https://doc.rust-lang.org/rust-by-example/variable_bindings.html
//
// From the page: "Rust provides type safety via 
// static typing. Variable bindings can be type
// annotated when declared. However, in most cases, 
// the compiler will be able to infer the type of
// the variable from the context, heavily reducing 
// the annotation burden."
//
// Continuing "Values (like literals) can be bound 
// to variables, using the let binding."
// 
// That last statement with `(like literals)` ins't
// super helpful for me since I don't think I 
// really know what a literal is at this point. 

fn main() {
    let alfa: i32 = 16;
    println!("The value in {}", alfa)
}
