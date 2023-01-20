// https://doc.rust-lang.org/rust-by-example/flow_control/while.html
//
// This is fizzbuzz
//
// TODO: Define FizzBuzz
//

fn main() {
    let mut counter = 0;

    while counter <= 20 {
        if counter % 15 == 0 {
            println!("fizzbuzz");
        } else if counter % 3 == 0 {
            println!("fizz")
        } else if counter % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", counter)
        }

        counter += 1;
    }
}


// TODO: Make sure that `%` has been 
// explained.
//

