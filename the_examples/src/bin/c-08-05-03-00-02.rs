// https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
//
// Binding
//
// You can also use binding to "destructure" enum variants, such as Option:
//


fn alfa() -> Option<u32> {
    Some(17)
}

fn main() {
    match alfa() {

        Some(match_number @ 17) => 
            println!("Matched value: {match_number}"),

        Some(match_number) =>
            println!("Other number: {match_number}"),

        _ => 
            println!("Got None")

    }
}


// I'm not sure if you can get to the `_` match
// or not. The use case is for None. I don't know
// if it's possible to pass that with a u32.




