// This is a follow up example from
// the prior one but uses fnMut.

fn create_alfa_fnmut() -> impl FnMut() {
    let text = "alfa".to_owned();

    move || println!("This is {text}")
}

fn main() {
    let mut alfa_fnmut = create_alfa_fnmut();

    alfa_fnmut();
}
