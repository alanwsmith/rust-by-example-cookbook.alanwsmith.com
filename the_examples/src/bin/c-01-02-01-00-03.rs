// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html

/////////////////////////////////////////////////////
// This fails

/*
struct Widget(i32);

fn main() {
    println!("Intentional Error {}", Widget(1717))
}
*/

// The reason that doesn't work is because you have
// to manually implement what's needed for it to 
// work. However, you can call a derive to get it to
// work automatically. 

////////////////////////////////////////////////////

#[derive(Debug)]
struct Widget(i32);

fn main() {
    println!("Automatic Debug Output {:?}", Widget(1717))
}

// OUTPUTS:
// Automatic Debug Output Widget(1717)

// This is definitly more complication than should be 
// at the start of the process given the `derive` call. 
// That type of thing needs it's own explination. 
// (I don't even know what to call it myself yet)


