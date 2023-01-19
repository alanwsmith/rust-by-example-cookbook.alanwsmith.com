fn main() {

    let alfa = (17, 24, 38);

    match alfa {
        (.., 38) => println!("Ends in 38"),
        _ => println!("Got something else")
    }

}


// TODO: figure out how to get a value in the middle
// when you use `..`. E.g. can you do (1,2,3,4), and
// then (..,..,3,..)
//
