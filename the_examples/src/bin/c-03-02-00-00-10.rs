// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
//
// This is straight from the example site:
//
// "Type aliases - If you use a type alias, you can 
// refer to each enum variant via its alias. This 
// might be useful if the enum's name is too long or 
// too generic, and you want to rename it."
//
// We haven't seen type aliases yet. So punting on this
// for now (it also doesn't have any output)


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}

