fn main() {

    let alfa = (17, 24, 38);

    match alfa {
        (17, .., 38) => println!("Got the endges"),
        _ => println!("Got something else")
    }

}


