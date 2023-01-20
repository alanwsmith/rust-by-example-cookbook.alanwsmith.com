// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// "Arrays are stack allocated" 
// That's the first time the stack has been mentioned
// and there's no context or explination. 
//
// This can definitly go later in the mix. Nothing
// at this stage is likely to require needing
// to know memory allocation. 

use std::mem;

fn main() {

    let foxtrot_array: [i32; 4] = [18, 22, 39, 43];

    println!(
        "Memory used {}", 
        mem::size_of_val(&foxtrot_array)
    );

}


// OUTPUT
// Memory used 16

// Bytes, I'm assuming
//
//
