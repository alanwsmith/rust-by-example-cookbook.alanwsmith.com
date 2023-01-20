// https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
//
// jono_codes: that form of format strings is going out of fashion, the book mus be out of date. The more recent approach is "The value is {count}"
//
//
//



fn main() {
    let mut count: i32 = 0;

    loop {
        count += 1;

        if count == 3 {
            continue;
        }

        if count == 5 {
            break;
        }
        println!("The value is {}", count)
    }
}


// This skips "3"
//
// TODO: write this up better. 

