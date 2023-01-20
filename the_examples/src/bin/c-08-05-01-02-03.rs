// Ignore the end of the array
//

fn main() {

    let alfa = [17, 24, 38];

    match alfa {
        [17, beta, ..] => 
            println!("Got {beta} ignore the rest"),

        _ => println!("Got something else")
    }

}
