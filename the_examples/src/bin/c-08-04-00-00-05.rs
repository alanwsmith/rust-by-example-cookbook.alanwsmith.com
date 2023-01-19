// https://doc.rust-lang.org/rust-by-example/flow_control/for.html
//
// This is .iter_mut()
//
// "This mutably borrows each element of the collection, allowing for the collection to be modified
// in place."
//

// fn main() {
//     let mut names = vec!["Bob", "Frank", "Ferris"];
//     for name in names.iter_mut() {
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }
//     println!("names: {:?}", names);
// }


fn main() {
    
    let mut letters = vec!["Alfa", "Bravo", "Charlie"];

    for letter in letters.iter_mut() {
        *letter = match letter {
            &mut "Bravo" => "BANANNA",
            _ => letter
        }
    }

    println!("{letters:?}")

}


// NOTE: In the match, you have to deal with 
// all cases which is whey the values are put
// back in place. 


// Note that the vec is made with `mut` which it wasn't
// in prior examples. 
