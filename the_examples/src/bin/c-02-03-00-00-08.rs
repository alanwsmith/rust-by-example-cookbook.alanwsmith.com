// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// This is looping throuhg an array 
// It also shows how to use get to access an
// index


fn main() {
    
    let alfa: [i32; 4] = [15, 28, 31, 49];

    for i in 0..alfa.len() {
        println!("Value {:?}", alfa.get(i))
    }
}

// This outputs `Some` values
//
// OUTPUT
// Value Some(15)
// Value Some(28)
// Value Some(31)
// Value Some(49)




