//  Arrays destruturing with matches:
//  you can skip slots with `_` so that
//  you don't have to destrcutre it:
//
// TODO: See if that also works with tuples

fn main() {

    let alfa = [17, 24, 38];

    match alfa {
        [17, _, beta] => 
            println!("Third value is: {beta}"),

        _ => println!("Got someting else")
    }

}
