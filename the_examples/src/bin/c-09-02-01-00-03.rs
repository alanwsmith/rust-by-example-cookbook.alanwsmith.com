// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html
//


fn main() {

    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("Count is {count}");
    };

    increment();

}

// This is a basic one showing how 
// to use a mutable variable inside
// a closure. 

