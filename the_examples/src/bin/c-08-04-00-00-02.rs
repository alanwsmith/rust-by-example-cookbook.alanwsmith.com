// https://doc.rust-lang.org/rust-by-example/flow_control/for.html
//
// This example uses `a..=b` which includes the values
// from both ends of the range

fn main() {

    for counter in 1..=10 {
        println!("The value is {counter}");
    }
}
