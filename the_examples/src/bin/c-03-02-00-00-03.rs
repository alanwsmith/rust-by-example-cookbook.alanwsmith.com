// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// Working on my own enum example that's less complicated
//
// This uses `unit-like` enum items that don't have
// a value. They are used to match against. 

#[allow(dead_code)]

#[derive(Debug)]
enum Container {
   Alfa,
   Bravo
}

fn main() {
    let thing = Container::Bravo;

    match thing {
        Container::Alfa => println!("Got Alfa"),
        Container::Bravo => println!("Got Bravo")
    };
}

