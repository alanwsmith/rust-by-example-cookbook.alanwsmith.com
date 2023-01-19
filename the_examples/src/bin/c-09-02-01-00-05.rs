fn main() {

    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("Count is {count}");
    };

    increment();
    increment();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

}


// TODO: Figure out what's happening here
// and write it up a little better. 




