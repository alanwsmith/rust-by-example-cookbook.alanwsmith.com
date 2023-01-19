// https://doc.rust-lang.org/rust-by-example/fn/methods.html
//


struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying {first} {second}")
    }
}


fn main() {
    let alfa = Pair(Box::new(1), Box::new(2));

    alfa.destroy();
}

// TODO: Look up `Box`. This is something 
// to see about in the rust book. 
//
// If you try calling `alfa.destroy()` again
// you get this error:
//
// error[E0382]: use of moved value: `alfa`

/*
  --> src/bin/c-09-01-00-00-02.rs:20:5
   |
16 |     let alfa = Pair(Box::new(1), Box::new(2));
   |         ---- move occurs because `alfa` has type `Pair`, which does not implement the `Copy` trait
17 |
18 |     alfa.destroy();
   |          --------- `alfa` moved due to this method call
19 |
20 |     alfa.destroy();
   |     ^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `alfa`
  --> src/bin/c-09-01-00-00-02.rs:8:16
   |
8  |     fn destroy(self) {
   |                ^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `the_examples` due to previous error
*/
