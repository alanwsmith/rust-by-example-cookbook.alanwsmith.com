// "structs - Similarly, a struct can be destructured as shown:"
//
// This is the copy straight from the web page
// that needs to be parsed down


struct Wrapper {
    x: (i32, i32),
    y: i32
}

fn main() {

    let alfa = Wrapper { x: (1, 2), y: 3 };

    match alfa {
        Wrapper { x: (1, b), y } =>
            println!("First of x is 1, b = {b}, y = {y}"),

        // you can destructure structs and rename the variables,
        // the order is not important
        Wrapper { y: 2, x: i } => 
            println!("y is 2, i = {i:?}"),

        // and you can also ignore some variables:
        Wrapper { y, .. } => 
            println!("y = {y}, we don't care about x")

    }
}

// NOTE: until adding the `{ y, .. }` the example 
// failed to compile. 


// The example had the struct definition in
// main. I moved it out becasue that's 
// where I see it most of the time and
// pretty sure that's where it was in earlier
// examples
//
