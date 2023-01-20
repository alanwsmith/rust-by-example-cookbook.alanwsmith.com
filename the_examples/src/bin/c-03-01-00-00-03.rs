// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Classis C structs (which are one of the three types
// of possible structs: Tuple, Classic C, and Unit)
//


struct ClassicStyle {
    start: i32,
    end: i32
}

fn main() {

    let alfa = ClassicStyle { start: 14, end: 29 };

    println!(
        "Start {} End {}", 
        alfa.start,
        alfa.end
    )
}
