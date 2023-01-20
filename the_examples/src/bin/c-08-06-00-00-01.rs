// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//
// if/let
//
// "For some use cases, when matching enums, match is awkward.
// For example:"


fn main() {

    let alfa = Some(15);

    match alfa{
        Some(bravo) => {
            println!("The value is {bravo}")
        }, 

        _ => {},
    }

}


// Note: This isn't the if/let, this is just the
// example showing a match sample. Not sure if
// that's the base way to do it. 
