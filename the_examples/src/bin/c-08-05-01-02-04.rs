// pull things into an array slice
// (we haven't seen those yet so not 
// really sure what they are)

fn main() {

    let alfa = [17, 24, 38];

    match alfa {
        [17, remaining @ ..] => println!("Remainder is {remaining:?}"),

        _ => println!("Got something else")
    }
}
