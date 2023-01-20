// You can define something and then 
// pull it out as a ref like this
// (again, I don't totally understand this
// yet)

fn main() {

    let alfa = 5;

    match alfa {
        ref value => 
            println!("Got {value}")
    }

}
