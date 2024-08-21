// thanks to https://t.me/terriblerust for this one!
// Larger struct = smaller size!

#![allow(dead_code)]
struct Foo {
    ptr: *const u8,
    num: i16,
}

struct Bar {
    ptr: *const u8,
    num: i16,
    _unused: bool,
}

enum Baz<T> {
    First(f64),
    Second(T),
}

fn main() {
    println!("{}", std::mem::size_of::<Baz<Foo>>()); // 24
    println!("{}", std::mem::size_of::<Baz<Bar>>()); // 16
}
