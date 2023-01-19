// this is a ref with a mutalbe
// variable. Still not sure how this
// works
//
// This is also using 

fn main() {

    let mut alfa = 7;

    match alfa {
        ref mut value => {
            println!("Value is {value}");
            *value  += 1;
            println!("New value is {value}");
        }
    }

}


// Note that the `*` dereferences it. Need
// to dig into that more
//
