// shadowing only happens in the scope
// where use/as is called. 


fn alfa() {
    println!("This is top level alfa()")
}

mod widget {
    pub fn alfa() {
        println!("This is widget::alfa()")
    }
}

fn main() {

    alfa();

    {
        use widget::alfa as alfa;
        alfa()
    }

    alfa();

}
