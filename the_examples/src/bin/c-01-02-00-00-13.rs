fn main() {
    println!(
        "Varible Zero Padded {base_number:0>padding$}",
        base_number=2121,
        padding=10
    )
}

// This uses the named parametered with an `$` after
// it to set the padding size.
