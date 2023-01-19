// 
//
// This is another copy stright out of the site
// that's more about type aliases
// 
// "The most common place you'll see this is in 
// impl blocks using the Self alias."
//
// Punting on this too since we haven't 
// learned about impl blocks yet. 
//
// It also doen't complie as is


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// The prolog says: "To learn more about enums and 
// type aliases, you can read the stabilization 
// report from when this feature was stabilized 
// into Rust." with this link:
//
// https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847


