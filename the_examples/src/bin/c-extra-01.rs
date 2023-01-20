
// Using this with the `!` covers everything
// in the scope (which in this case is everying)
#![allow(unused_variables)]


#[allow(dead_code)]
fn bravo() {

}

// Using this without the `!` applies to just
// the following block
// #[allow(unused_variables)]
fn main() {
    let alfa =1234;
    // println!("{}", alfa)
}

