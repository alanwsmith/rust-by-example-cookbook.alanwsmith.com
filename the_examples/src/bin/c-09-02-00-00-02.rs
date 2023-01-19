fn main() {

    let alfa = |value| value + 1;

    let bravo = alfa(7);

    println!("Bravo is {bravo}")
}


// This is an example where the input
// and return types are both inferred
//
// That is this:
//
// |value| value + 1
//
// Vs this which is explicit
//
// |value: i32| value + 1
//
//
