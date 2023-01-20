// https://doc.rust-lang.org/rust-by-example/hello/print.html
// This example isn't actually on the page
// but it shows how to use both positional and
// named arguments. Not something to show in the 
// initial set. 

fn main() {
    println!(
        "Postion {0} And Named {example}",
        "alfa",
        example="bravo"
    )
}

// Doesn't feel like something you'd want to do
// most of the time, but it works. 

// OUTPUT:
// Postion alfa And Named bravo
