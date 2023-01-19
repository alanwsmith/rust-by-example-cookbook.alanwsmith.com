// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//


fn main() {

    let alfa: Option<i32> = None;
    let bravo: bool = false;

    if let Some(charlie) = alfa {
        println!("Matched {charlie}")
    } else if bravo {
        println!("No match and bravo is true")
    } else {
        println!("No match and bravo is false")
    }

}


