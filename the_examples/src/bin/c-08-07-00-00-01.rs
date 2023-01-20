// https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html
//
// while let
//
// "Similar to if let, while let can make awkward 
// match sequences more tolerable."

fn main() {

    let mut alfa: Option<i32> = Some(0);

    while let Some(value) = alfa {
        if value > 9 {
            println!("It's more than 9");
            alfa = None;
        } else {
            println!("Current value is {value}");
            alfa = Some(value + 1);
        }
    }

}


// It looks like `while` breaks on `None`
// TODO: Show an explict while loop with a 
// None for the break condition. 
