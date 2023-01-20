// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// Pull values from a tuple


fn main() {

    let tuple_example = (43, 13, 57);

    println!(
        "Values {} {} {}", 
        tuple_example.0, 
        tuple_example.1, 
        tuple_example.2, 
    )

}

