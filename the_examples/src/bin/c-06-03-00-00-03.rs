// https://doc.rust-lang.org/rust-by-example/conversion/string.html
//
// This is the turbo fish example
//

fn main() {
    let alfa: &str = "5";

    let bravo = alfa.parse::<i32>().unwrap();

    let charlie = bravo + bravo;

    println!("The value is {}", charlie)
}
