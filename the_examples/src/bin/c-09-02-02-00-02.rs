// I don't lik ethis example, but it works
//
// TODO: write up more about how this works.
//

fn apply_to_3<C>(f: C) -> i32 where
    C: Fn(i32) -> i32 {
        f(3)
    }

fn main() {
    let alfa = |value| 2 * value;
    println!("3 doubled: {}", apply_to_3(alfa));
}

