// https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
//
// Useing `mut` lets you change a variable by 
// making it mutalbe. 
//



fn main() {
    let mut alfa: i32 = 123;

    println!("The value is {}", alfa);

    alfa = 456;

    println!("The value is {}", alfa);
}


// NOTE: If you don't use the first value
// in the variable, you'll get a warning
// about it not being used. 
//



