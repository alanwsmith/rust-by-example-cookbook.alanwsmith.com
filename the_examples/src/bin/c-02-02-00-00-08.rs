// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// You can pull stuff out of tuples by destructring them


fn main() {
    let content_tuple = (64, 26, 71, 13);

    let (alfa, bravo, charlie, delta) = content_tuple;

    println!(
        "Values {} {} {} {}", 
        alfa, 
        bravo,
        charlie,
        delta
    );
}
