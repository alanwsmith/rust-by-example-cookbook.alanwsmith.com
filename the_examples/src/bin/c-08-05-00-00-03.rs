fn main() {

    let alfa = 3;

    match alfa {
        1 => println!("Got 1"),
        2 | 3 | 4 => println!("Got 2, 3, or 4"),
        _ => println!("Got something else")
    }

}
