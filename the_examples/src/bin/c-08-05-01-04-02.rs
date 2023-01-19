// This is a defererenced value in a match
// not sure how that would work with more 
// than one thing, so not sure why match 
// is used. 

fn main() {

    let alfa = &7;

    match *alfa {
        value => println!("Deferenced value {value}")
    }

}



