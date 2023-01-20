// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Making a new instance of a strcuct by 
// with the update syntax
//


struct Example {
    alfa: i32,
    bravo: i32,
    charlie: i32
}

fn main() {
    let first = Example {
        alfa: 15, 
        bravo: 24, 
        charlie: 36
    };

    let second = Example { bravo: 9999, ..first };

    println!(
        "First {} {} {}",
        first.alfa,
        first.bravo,
        first.charlie
    );

    println!(
        "Second {} {} {}",
        second.alfa, 
        second.bravo, 
        second.charlie
    );

}

// OUTPUTS
// First 15 24 36
// Second 15 9999 36
//
