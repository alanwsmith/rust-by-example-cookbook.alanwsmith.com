// https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html
//
// Input functions
//

/*
   Since closures may be used as arguments, you 
   might wonder if the same can be said about 
   functions. And indeed they can! If you 
   declare a function that takes a closure as 
   parameter, then any function that satisfies 
   the trait bound of that closure can be passed 
   as a parameter.
*/

fn call_me<F: Fn()>(f: F) {
    f();
}

fn alfa_function() {
    println!("This is alfa_funciton");
}

fn main() {
    let bravo_closure = || {
        println!("This is bravo_closure");
    };

    call_me(alfa_function);
    call_me(bravo_closure);
}

// As an additional note, the Fn, FnMut, and 
// FnOnce traits dictate how a closure captures 
// variables from the enclosing scope.


