// https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html
//
// Guards - A match guard can be added to filter the arm.
//

// This is the example from the website:
//

enum Temperature {
    Celsius(i32), 
    Fahrenheit(i32)
}



fn main() {

    let temperature = Temperature::Celsius(35);

    match temperature {

        Temperature::Celsius(degrees) if degrees > 30 =>
            println!("{degrees} is above 30C"),

        Temperature::Celsius(degrees) =>
            println!("{degrees} is 30C or below"),

        Temperature::Fahrenheit(degrees) if degrees > 86 =>
            println!("{degrees} is above 86F"),

        Temperature::Fahrenheit(degrees) =>
            println!("{degrees} is 86F or below")

    }

}

// "Note that the compiler won't take guard conditions 
// into account when checking if all patterns are 
// covered by the match expression."
// 


// TODO: Show error messages like this is you don't get 
// all the variations:
//


/*
note: `Temperature` defined here
  --> src/bin/c-08-05-02-00-01.rs:10:5
   |
9  | enum Temperature {
   |      -----------
10 |     Celsius(i32),
   |     ^^^^^^^ not covered
11 |     Fahrenheit(i32)
   |     ^^^^^^^^^^ not covered
   = note: the matched value is of type `Temperature`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
   |
22 ~             println!("{degrees} is above 30"),
23 ~         Celsius(_) | Fahrenheit(_) => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `the_examples` due to previous error
*/
