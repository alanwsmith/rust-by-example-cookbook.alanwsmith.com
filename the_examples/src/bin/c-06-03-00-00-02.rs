// https://doc.rust-lang.org/rust-by-example/conversion/string.html
//
// From the page:
//
/* 
One of the more common types to convert a string into is 
a number. The idiomatic approach to this is to use the 
parse function and either to arrange for type inference 
or to specify the type to parse using the 'turbofish' 
syntax. Both alternatives are shown in the following example.

This will convert the string into the type specified as 
long as the FromStr trait is implemented for that type. 
This is implemented for numerous types within the standard 
library. To obtain this functionality on a user defined 
type simply implement the FromStr trait for that type.
*/

// This references the turbofish, but doestn' say 
// what it is. 
//
// Acutally, this is just the basic parse, the 
// next example will be the turbofish parse
//

fn main() {
    let alfa: &str = "5";
    let bravo: i32 = alfa.parse().unwrap();
    let charlie = bravo + bravo;

    println!("The value is {}", charlie)
}


// TODO: Define what &str is
//
// TODO: define what .unwrap() does


