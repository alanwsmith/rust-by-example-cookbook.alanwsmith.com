// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
//
// pointers/ref - "For pointers, a distinction needs to be made between destructuring and
// dereferencing as they are different concepts which are used differently from languages like
// C/C++."
//
// Dereferencing uses *
// Destructuring uses &, ref, and ref mut
//
// Haven't heard about pointers and ref yet I don't think
//
// I also don't understand why this is using a match
// since there is only one thing and it looks like
// it always matches that one thing.


fn main() {

    let alfa = &4;

    match alfa {
        &any_value => println!("Destructed value {any_value}")
    }

}

// "This assign a reference of type `i32`. 
// The `&` signifies there is a reference 
// being assigned."

// Then: "If `reference` is pattern matched 
// against `&val`, it results in a comparison 
// like:
// `&i32`
// `&val`
// We see that if the matching `&`s are dropped, 
// then the `i32` should be assigned to `val`."
//
// I don't really understand that.


