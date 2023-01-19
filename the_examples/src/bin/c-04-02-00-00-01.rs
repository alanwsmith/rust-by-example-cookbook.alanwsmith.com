// https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
//
// Blocks are setup with `{...}`. Things that
// are inside the `{}` can't be accessed outside them
// TODO: explain more explicitly about scope. 

// This works:
//

fn main() {
    {
        let alfa: i32 = 123;
        println!("The value is {}", alfa)
    }
}

// This will error and crash:
//

// fn main() {
//     {
//         let alfa: i32 = 123;
//     }
//     println!("The value is {}", alfa)
// }



