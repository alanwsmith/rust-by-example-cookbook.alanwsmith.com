// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// Working on my own enum example that's less complicated
//
// This uses `unit-like` enum items that don't have
// a value. 
//
// The `#[allow(dead_code)]` is necessary to prevent a warning
// that only one of the items of the emum is used. 

#[allow(dead_code)]

#[derive(Debug)]
enum Container {
   Alfa,
   Bravo
}

fn main() {
    let thing = Container::Bravo;

    println!("Value {:?}", thing)
}

