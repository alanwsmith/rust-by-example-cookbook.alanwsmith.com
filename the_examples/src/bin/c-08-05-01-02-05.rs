// grab the first and last and then
// pull the middle into a slice

fn main() {

    let alfa = [17, 24, 38];


    match alfa {
        [ first, middle @ .., last] => println!(
            "{first} {last} {middle:?}"
        ),

        _ => println!("Got something else")
    }

}
