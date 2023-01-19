// https://doc.rust-lang.org/rust-by-example/flow_control/for.html
//

/*
into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
*/



fn main() {
    let letters = vec!["Alfa", "Bravo", "Charlie"];

    for letter in letters.into_iter() {
        match letter {
            "Bravo" => println!("Got Bravo"),
            _ => println!("Not Bravo. Got {letter}")
        }
    }
}

// Difference is `"Bravo"` vs `&"Bravo"`. Need to
// clarify that. The `&` is for a reffercence. 
// Without it, is used to compare with the value
// itself. Which is what you get from `.into_iter()`

// If you tried to print stuff out from the `letters`
// vec at the end, you'll get an error like:
//
/*
error[E0382]: borrow of moved value: `letters`
  --> src/bin/c-08-04-00-00-04.rs:19:16
   |
10 |     let letters = vec!["Alfa", "Bravo", "Charlie"];
   |         ------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
11 |
12 |     for letter in letters.into_iter() {
   |                           ----------- `letters` moved due to this method call
...
19 |     println!("{letters:?}");
   |                ^^^^^^^ value borrowed here after move
*/

