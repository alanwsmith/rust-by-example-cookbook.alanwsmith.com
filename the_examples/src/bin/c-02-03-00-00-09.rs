// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// This is the array loop example copied directly from
// the site. It intentionally pushes past the array 
// lenght by one and uses match to check the value type
// that gets returned. TPBD on how frequently that's 
// necessary

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

// OUTPUTS
//
// 0: 1
// 1: 2
// 2: 3
// 3: 4
// 4: 5
// Slow down! 5 is too far!



