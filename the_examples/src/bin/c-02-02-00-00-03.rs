// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// Tuples can have multiple types in them

fn main() {
    let alfa: i32 = 43;
    let bravo: f32 = 0.72;
    let charlie: u32 = 67;

    let tuple_example = (alfa, bravo, charlie);

    println!(
        "Values: {} {} {}", 
        tuple_example.0, 
        tuple_example.1, 
        tuple_example.2
    )

}
