// https://doc.rust-lang.org/rust-by-example/flow_control/for.html
//
/*
for and range

The for in construct can be used to iterate 
through an Iterator. One of the easiest ways 
to create an iterator is to use the range 
notation a..b. This yields values from a 
(inclusive) to b (exclusive) in steps of one.
*/

fn main() {

    for counter in 1..10 {
        println!("The value is {counter}");
    };

}


// TODO: Pull in FizzBuzz from the page
// for a secondary example. 
