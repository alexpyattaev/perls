// thanks to https://t.me/terriblerust for this one!
pub mod prelude {
    mod private {
        pub trait Trait { fn method(&self) {} }
        impl Trait for i32 {}
    }
    
    pub use private::Trait as _;
}

// use prelude::Trait; // doesn't work, no `Trait` in `prelude`
// use prelude::_;     // doesn't work, can't import `_`
use prelude::*;

fn main() {
    // works! methods from `Trait` are accessible
    42.method();
}
