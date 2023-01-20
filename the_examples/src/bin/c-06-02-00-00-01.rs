// https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
//
// TryFrom
//
// From the page "Similar to From and Into, TryFrom 
// and TryInto are generic traits for converting 
// between types. Unlike From/Into, the TryFrom/TryInto 
// traits are used for fallible conversions, and as 
// such, return Results."
//
// NOTE: We haven't learned what Result is yet. This
// also has `#[derive(Debug, PartialEq)]` where we
// haven't seent `PartialEq` before. This also
// uses `assert_eq!` which we haven't seen before. 
//

use std::convert::TryFrom;

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
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    println!("here")
}


