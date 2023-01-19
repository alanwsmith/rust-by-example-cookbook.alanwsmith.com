// https://doc.rust-lang.org/rust-by-example/flow_control/match.html
//
// match with numbers and a `_`
//

fn main() {

    let alfa: i32 = 3;

    match alfa {
        1 => println!("Matched 1"),
        2 => println!("Matched 2"),
        3 => println!("Matched 3"),
        4 => println!("Matched 4"),
        _ => println!("Got something else")
    }

}
