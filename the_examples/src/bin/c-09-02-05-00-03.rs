// This is the next example with
// fnOnce

fn create_alfa_fnonce() -> impl FnOnce() {
    let text = "alfa".to_owned();

    move || println!("This is {text}")
}

fn main() {
    let alfa_fnonce = create_alfa_fnonce();

    alfa_fnonce();
}
