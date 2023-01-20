// match destructure tuple
//  
// Not sure we've done tuples yet?
//

fn main() {
    let alfa = (17, 24, 38);

    match alfa {
        (17, x, y) => println!("First is 12, then {x} {y}"),
        _ => println!("Got something else")
    }

}
