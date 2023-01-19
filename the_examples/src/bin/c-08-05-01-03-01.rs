// https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html
//
// emums - An enum is destructured similarly:
//
// Not sure if we've done enums yet. This is 
// a cut down version from the web site
//

#[allow(dead_code)]
enum Color {
    RGB(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::RGB(r, g, b) =>
            println!("RGB {r} {g} {b}"),

        Color::CMYK(c, m, y, k) =>
            println!("CMYK {c} {m} {y} {k}!")
    }
}


// "Doesn't need another arm because all variants have been examined"
//
// TODO: Change this so that it isn't using color stuff
//
// TODO: Change this so that it isn't using color stuff
