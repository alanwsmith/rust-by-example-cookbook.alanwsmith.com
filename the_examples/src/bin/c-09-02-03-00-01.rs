// https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html
//
// Type anonymity
//
// TODO: Write up a version of the page 
// you can understand without hurting your brain. 
//
//
//


fn apply<F>(f: F) where
    F: Fn() {
        f();
    }

fn main() {
    let alfa = 7;

    let bravo = || println!("The value is {}", alfa);
    
    apply(bravo)
}

// TODO: write up the way `<F>` allows for the 
// first `F` to exist that lets it be used. 
// This is the binding stuff. 

// NOTE: This and a few of the earlier exercises
// are showing possibilities but not necessarily
// how stuff would be done in practice. 

// Also look at Fn, FnMut, and FnOnce. 
//
//

