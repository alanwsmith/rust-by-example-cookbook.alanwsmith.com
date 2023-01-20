// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//
// "if let is cleaner for this use case and in 
// addition allows various failure options to be specified"
//

fn main() {

    let alfa = Some(7);

    if let Some(bravo) = alfa {
        println!("Matched {bravo}")
    }

}


