// The prior example uses `&4` to create the
// ref, it's also possible to use `ref` on
// `let` like this:

fn main() {

    let ref alfa = 9;

    match alfa {
        ref value => println!("Got reference {value}")
    }

}


// TODO: Figure out what the "rustic" way
// to do references is (i.e. with `&` or
// with `ref`)
//
