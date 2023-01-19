// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// This uses `unit-like` enum items that don't have
// a value. They are used to match against. 

#[allow(dead_code)]

enum Container {
   Alfa,
   Bravo
}

fn main() {

    // assign an enum to thing
    let thing = Container::Bravo;

    let response = match thing {
        Container::Alfa => "it's alfa",
        Container::Bravo => "it's bravo"
    };

    println!("Value {}", response);
}

