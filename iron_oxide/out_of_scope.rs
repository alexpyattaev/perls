// thanks to https://t.me/terriblerust for this one!
mod a {
    pub trait Foo {
        fn bar(&self);
    }
}

impl a::Foo for () {
    fn bar(&self) {
        mod hide { // We're in a new module...
            pub fn baz() {
                // ...so why is `Foo` in scope here?
                ().bar()
            }
        }
        hide::baz();
    }    
}
