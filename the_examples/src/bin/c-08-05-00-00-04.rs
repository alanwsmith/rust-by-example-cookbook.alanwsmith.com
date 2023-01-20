// 
//
// Match in range
//

fn main() {

    let alfa = 23;

    match alfa {
        1 => println!("Got 1"),
        2..=30 => println!("Got in the range"),
        _ => println!("Got something else")
    }

}
