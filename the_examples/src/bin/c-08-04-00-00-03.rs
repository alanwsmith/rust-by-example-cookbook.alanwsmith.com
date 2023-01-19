// https://doc.rust-lang.org/rust-by-example/flow_control/for.html
//

/*
for and iterators
The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators.

into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.

iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
*/


fn main() {
    let letters = vec!["Alfa", "Bravo", "Charlie"];
    
    for letter in letters.iter() {
        match letter {
            &"Bravo" => println!("Got Bravo"),
            _ => println!("Not Bravo. Is {letter}")
        }
    }

    println!("letters: {letters:?}");

}

// The last println works because we are using
// `.iter()` which means the data doesn't
// get pulled out of the Vec like it does if
// you use `.into_iter()` which is the next 
// example

// TODO: explain `&` in front of `&"Bravo"` 
//
