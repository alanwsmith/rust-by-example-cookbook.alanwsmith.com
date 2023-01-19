// https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html
//
//

#![allow(dead_code)]

enum Alfa {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

fn main() {
    let color_value = Alfa::Red as i32;

    println!("The color value #{:06x}", color_value);
}
