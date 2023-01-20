// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// I'm not sure if this is the standard way to 
// pull a value out of an emum. TODO: Needs 
// investigation. Feels like there should be 
// a way to access it without match

#[allow(dead_code)]

enum Chooser {
    Alfa(i32, i32), 
    Bravo(i32, i32)
}

fn main() {
    let thing = Chooser::Bravo(15, 27);

    let the_value = match thing {
        Chooser::Alfa(a, b) => format!("Alfa {} {}", a, b),
        Chooser::Bravo(a, b) => format!("Bravo {} {}", a, b)
    };

    println!("Value {}", the_value)
}

// TODO: Show example with `_` like:
// Chooser::Alfa(_, value) => format!("Alfa {}", value)
// where the first value is skipped via a placeholder


