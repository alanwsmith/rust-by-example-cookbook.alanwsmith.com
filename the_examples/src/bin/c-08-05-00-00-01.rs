// https://doc.rust-lang.org/rust-by-example/flow_control/match.html
//
// match - From the page: "Rust provides pattern matching via the 
// match keyword, which can be used like a C switch. The first 
// matching arm is evaluated and all possible values must be covered."
//

fn main() {

    let alfa: bool = true;

    match alfa {
        true => println!("Matched true"),
        false => println!("Matched false")
    }

}
