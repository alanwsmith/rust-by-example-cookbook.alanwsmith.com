// https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
//
// Binding
//
// "Indirectly accessing a variable makes it impossible 
// to branch and use that variable without re-binding. 
// match provides the @ sigil for binding values to names"
//
// I don't really know what that means. 
//

fn age() -> i32 {
    15
}

fn main() {
    match age() {

        0 => 
            println!("Not yet 1"),

        years @ 1 ..= 12 =>
            println!("Kid age {years}"),

        years @ 13 ..= 19 =>
            println!("Teen age {years}"),
            
        years => 
            println!("Adult age {years}")

    }

}
