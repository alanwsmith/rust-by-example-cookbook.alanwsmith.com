// https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html
//
// Another one that feels like it should be pushed later since
// I don't know what a Vec is yet. 

use std::fmt;

struct Alfa(Vec<i32>);

impl fmt::Display for Alfa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { 
                write!(f, ", ")?;
            }
            write!(f, "Item {} is {}", count, v)?;
        }
        // This needs to be here otherwise it 
        // fails with the `?` on the prior one. 
        // Wouldn't be surprised to find a better
        // way around that.
        write!(f, "")
    }
}


fn main() {
    let v = Alfa(vec![37, 43, 57]);
    println!("{}", v);
}

// TODO: Figure out how all the self stuff works 
// with the trait

// OUTPUTS:
// Item 0 is 37, Item 1 is 43, Item 2 is 57



