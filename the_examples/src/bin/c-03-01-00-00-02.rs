// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Tuple structs, which are, basically, named tuples.
// They are one of three types of structs (tuple, 
// classic C structs, and Unit structs)
//

struct TupleStruct(i32, i32, i32);

fn main() {

    let alfa = TupleStruct(19, 25, 36);

    println!(
        "Values {} {} {}", 
        alfa.0,
        alfa.1,
        alfa.2,
    )
}

// The values of the tuple don't all have to 
// be of the same type. 

// OUTPUT
// Values 19 25 36


