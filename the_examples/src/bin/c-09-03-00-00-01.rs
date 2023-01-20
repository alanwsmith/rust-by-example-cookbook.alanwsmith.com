// https://doc.rust-lang.org/rust-by-example/fn/hof.html
//
// Higher Order Functions
//

/* From the site

Rust provides Higher Order Functions (HOF). These 
are functions that take one or more functions 
and/or produce a more useful function. HOFs and 
lazy iterators give Rust its functional flavor.

*/

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let upper = 100;

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    
    println!("Value {sum_of_squared_odd_numbers}")

}

// TODO: See if there's a way to reduce the example
// by removing the squared part or otherwise
// just simplify it. 



