// https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
//
//

/*
Nesting and labels

It's possible to break or continue outer loops 
when dealing with nested loops. In these cases, 
the loops must be annotated with some `'label`, and 
the label must be passed to the break/continue statement.
*/

fn main() {

    let mut alfa: i32 = 0;
    let mut bravo: i32 = 0;

    'outer: loop {
        alfa += 1;

        loop {
            bravo += 1;

            if bravo == 5 {
                break 'outer;
            }

            println!("Alfa  {}", alfa);
            println!("Bravo {}", bravo);
        }
    }
}

// TODO: think about using `continue` on 
// outer for this as well. Or another version
//
