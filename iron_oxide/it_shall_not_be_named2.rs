pub mod prelude {
    mod private {
        pub trait Trait { fn method(&self) {println!("1")} }
        impl Trait for i32 {}
    }
    
    pub use self::private::Trait as _;
}

pub mod prelude2 {
    mod private {
        pub trait Trait2 { fn method(&self) {println!("2")} }
        impl Trait2 for i32 {}
    }
    
    pub use self::private::Trait2 as _;
}

//providing conflicting declarations 
use prelude::*;
use prelude2::*;

fn main() {
    // Now you can not make this work!
    42.method();
    // or this...
    Trait::method(&42);
}
