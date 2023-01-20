// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// This is the code they have for a Unit struct, but 
// I can't tell what you'd actually do with it. 
//
// The page says:
// Unit structs, which are field-less, are useful for generics.
//

#[derive(Debug)]
struct SomeUnit;

fn main() {
    let alfa_unit = SomeUnit;
    println!("The value {:?}", alfa_unit)
}
