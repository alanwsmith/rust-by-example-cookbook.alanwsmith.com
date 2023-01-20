// TODO: Show off dbg


fn main() {

    let alfa = |value: i32| {
        value + 1
    };

    let charlie = alfa(7);

    dbg!(alfa(dbg!(charlie)));

}
