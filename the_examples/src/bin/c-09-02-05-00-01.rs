// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html
//
// Closures as output parameters
//
// This is more advanced stuff that could 
// be pushed later in the mix

/* From the site:

Closures as input parameters are possible, so 
returning closures as output parameters should 
also be possible. However, anonymous closure 
types are, by definition, unknown, so we have 
to use impl Trait to return them.

The valid traits for returning a closure are:

- Fn
- FnMut
- FnOnce

Beyond this, the move keyword must be used, 
which signals that all captures occur by value. 
This is required because any captures by reference 
would be dropped as soon as the function exited, 
leaving invalid references in the closure.

*/

fn create_alfa_fn() -> impl Fn() {
    let text = "alfa".to_owned();

    move || println!("This is {text}")
}


fn main() {
    let alfa_fn = create_alfa_fn();

    alfa_fn();
}


// This is the `Fn` example


