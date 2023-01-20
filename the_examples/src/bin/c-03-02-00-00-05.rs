// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// tuple like enums
// I don't see a way to access the values in side an
// enum without match. I'll show that in another example

#[allow(dead_code)]

#[derive(Debug)]
enum Chooser {
    Alfa(i32, i32), 
    Bravo(i32, i32)
}

fn main() {
    let thing = Chooser::Bravo(15, 27);
    println!("Value {:?}", thing)
}
