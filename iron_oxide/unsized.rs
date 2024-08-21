// thanks to https://t.me/terriblerust for this one!
// it compiles. Clearly, string literals are not Sized.

#![allow(dead_code)]
fn lol() -> &'static impl ?Sized {
    "lol"
}
