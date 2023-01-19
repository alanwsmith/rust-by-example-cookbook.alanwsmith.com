// https://doc.rust-lang.org/rust-by-example/fn.html
//
// Functions
//
// From the site:
// "Functions are declared using the fn keyword. 
// Its arguments are type annotated, just like variables, 
// and, if the function returns a value, the return 
// type must be specified after an arrow ->."
//
// "The final expression in the function will be used 
// as return value. Alternatively, the return statement 
// can be used to return a value earlier from within 
// the function, even from inside loops or if statements."
//
// This is a straight copy from the example 


fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(left_side: u32, right_side: u32) -> bool {

    if right_side == 0 {
        return false;
    }

    left_side % right_side == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }

}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n)
    }
}


// Functions that "don't" return a value, actually return the unit type `()`
//
// TODO: make examples of this that aren't fizzbuzz
// and just show the features
//
