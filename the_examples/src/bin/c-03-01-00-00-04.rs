// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Destructure a tuple struct
//

struct BravoStruct(i32, i32, i32);

fn main() {

    let bravo = BravoStruct(11, 27, 34);

    let BravoStruct(a, b, c) = bravo;

    println!("Values {} {} {}", a, b, c)
}


