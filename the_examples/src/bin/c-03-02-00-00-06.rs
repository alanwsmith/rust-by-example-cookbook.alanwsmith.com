// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// tuple like enums
// using match to show the values inside the enum

#[allow(dead_code)]

#[derive(Debug)]
enum Chooser {
    Alfa(i32, i32), 
    Bravo(i32, i32)
}

fn main() {
    let thing = Chooser::Bravo(15, 27);

    match thing {
        Chooser::Alfa(a, b) => println!("Alfa {} {}", a, b),
        Chooser::Bravo(a, b) => println!("Bravo {} {}", a, b)
    }
}


