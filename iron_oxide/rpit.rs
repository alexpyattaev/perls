trait Wat {}

impl Wat for String {}

struct Thing {}

impl Thing {
    fn clone_thing2(&self) -> impl Wat {
        String::from("wat")
    }
}

trait Trait {
    fn clone_thing(&self) -> impl Wat;
}

impl Trait for Thing {
    fn clone_thing(&self) -> impl Wat{
        String::from("wat")
    }
}

fn main() {
    let obj = Thing {};
//    let x = obj.clone_thing(); //will not build, but why?
    let x = obj.clone_thing2();
    let mut obj = obj;
}

