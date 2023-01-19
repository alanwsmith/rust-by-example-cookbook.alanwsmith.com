// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// enum example with functions
//

#[allow(dead_code)]

enum Choice {
    Alfa (i32, i32, i32),
    Bravo {
        first: i32,
        second: i32
    }
}

fn process_alfa(a: i32, b: i32, c: i32) {
    println!("Alfa {} {} {}", a, b, c);
}

fn process_bravo(first: i32, second: i32) {
    println!("Bravo {} {}", first, second);
}

fn main() {
    let thing = Choice::Alfa(17, 25, 36);

    match thing {
        Choice::Alfa(a, b, c) => {
            process_alfa(a, b, c)
        },

        Choice::Bravo { first, second } => {
            process_bravo(first, second)
        }
    }
}

// OUTPUT
// Alfa 17 25 36
//
// TODO: Come up with better names for all 
// the things
//
// TODO: Figure out if there's a way to 
// setup so you don't have to use the allow
// dead code.


