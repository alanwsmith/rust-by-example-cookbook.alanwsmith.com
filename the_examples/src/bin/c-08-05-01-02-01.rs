// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_slice.html
//
// arrays/slices - Like tuples, arrays and slices can be destructured this way:
//

fn main() {

    let alfa = [17, 24, 38];

    match alfa {

        [17, beta, charlie] =>
            println!("Start 17, then {beta}, {charlie}"),

        _ => println!("Got something else")

    }

}
