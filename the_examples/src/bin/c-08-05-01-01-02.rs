fn main() {

    let alfa = (17, 24, 38);

    match alfa {
        (17, ..) => println!("Starts with 17 the rest is skipped"),
        _ => println!("Got something else")
    }

}
