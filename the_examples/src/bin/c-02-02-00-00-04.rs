// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// Tuples can have tuples inside them

fn main() {
    let alfa: i32 = 61;
    let bravo: i32 = 23;
    let charlie: i32 = 46;
    let delta: i32 = 71;

    let sub_tuple_1 = (alfa, bravo);
    let sub_tuple_2 = (charlie, delta);

    let tuple_of_tuples = (sub_tuple_1, sub_tuple_2);

    println!("Details {:?}", tuple_of_tuples);

}


