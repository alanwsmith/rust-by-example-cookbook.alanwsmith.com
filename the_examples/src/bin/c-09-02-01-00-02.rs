fn main() {

    let color = String::from("blue");

    let alfa = || println!("Value is {color}");

    alfa();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

}


// Figure out a better way to explain what's 
// happening here once you figure out what's 
// going on and can do it without inline 
// comments

