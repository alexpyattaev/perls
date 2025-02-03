
#[repr(u8)]
pub enum FooBar {
    Foo(i32),
    Bar(String),
}

fn main() {
    println!("{}", FooBar::Foo as u8);
    println!("{}", FooBar::Bar as u8);
}
