fn main() {

    use std::mem;

    // A non-copy type.
    let alfa = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("Alfa is {alfa:?}");
        mem::drop(alfa)
    };

    // `consume` consumes the variable so this can only be called once.
    consume();

}

// I don't understand what's happening here
// This is just a copy of with the comments
// left in place to try to figure it out. 
//
// The next example shows trying to call
// `consume()` twice which results in an 
// error
//
