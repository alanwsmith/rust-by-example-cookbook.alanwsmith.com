// https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
//
// This is TryInto
//
// Same text as prior page, also probably going
// to skip this in the first runthru. 
//

use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
}

// Yeah, this feels like a lot of overhead
// for an early part
