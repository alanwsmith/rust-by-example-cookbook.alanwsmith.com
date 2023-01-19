// This is the same as the prior example
// but with calling `consume()` twice
// which results in the error listed
// below. 

fn main() {

    use std::mem;

    let alfa = Box::new(3);

    let consume = || {
        println!("Alfa is {alfa:?}");
        mem::drop(alfa)
    };

    consume();
    consume();
}


/*
error[E0382]: use of moved value: `consume`
  --> src/bin/c-09-02-01-00-07.rs:18:5
   |
17 |     consume();
   |     --------- `consume` moved due to this call
18 |     consume();
   |     ^^^^^^^ value used here after move
   |
note: closure cannot be invoked more than once because it moves the variable `alfa` out of its environment
  --> src/bin/c-09-02-01-00-07.rs:14:19
   |
14 |         mem::drop(alfa)
   |                   ^^^^
note: this value implements `FnOnce`, which causes it to be moved when called
  --> src/bin/c-09-02-01-00-07.rs:17:5
   |
17 |     consume();
   |     ^^^^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `the_examples` due to previous error
*/


