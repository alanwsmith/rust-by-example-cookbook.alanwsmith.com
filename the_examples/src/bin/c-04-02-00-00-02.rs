// https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
//
// Variable shadowing
//

fn main() {

    let alfa: i32 = 123;

    {
        println!("Value at A {}", alfa);

        let alfa: i32 = 456;

        println!("Value at B {}", alfa);
    }

    println!("Value at C {}", alfa);

    let alfa: i32 = 789;

    println!("Value at D {}", alfa);

}



