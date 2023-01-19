// https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
//
// Using if statements to set a variable
//

fn main() {

    let alfa: i32 = 123;

    let bravo: &str = 
        if alfa == 0 {
            "zero"
        } else {
            "not zero"
        };
    
     println!("The value is {}", bravo)

}

// TODO: show a && for doing two 
// if comparisons. Also show the rest
// of the comparison operators. 
//
// TODO: Put in explicit note that
// you need to have the `;` at
// the end of the let statement 
// because it's always required 
// there.



