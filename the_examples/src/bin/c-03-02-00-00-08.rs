// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// enum with c-style structs
//

enum Wrapper {
    Alfa { first: i32, second: i32 },
    Bravo { third: i32, fourth: i32 }
}

fn main() {
    let thing = Wrapper::Alfa { first: 17, second: 27 };

    match thing {
        Wrapper::Alfa { first, second } => {
            println!("Values {} {}", first, second)
        }
        Wrapper::Bravo { third, fourth} => {
            println!("Values {} {}", third, fourth)
        }
    }
}
