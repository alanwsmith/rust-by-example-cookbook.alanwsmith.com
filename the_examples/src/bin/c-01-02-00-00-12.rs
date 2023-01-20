fn main() {
    // Start with this:
    // println!("Variable Zero Padded {}", 5678);

    // Then change to this.
    println!("Variable Zero Padded {:0>1$}", 5678, 10);

}

// This pads with a variable number of zeros. 
// The number comes from the `1` position (which
// is the second parameter) to determine hoe many
// zeros to use. This is done by adding `$` after 
// the postion index (e.g. `1` in this case)
//
// This is a bit confusig probably better to use
// the named ones that we'll do next. 



