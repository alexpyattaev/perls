// thanks to https://t.me/terriblerust for this one!
use std::ops::Not;

fn broke(is_cool: bool) { dbg!(is_cool); }

struct Cool;
impl Not for Cool {
    type Output = Option<Cool>;
    fn not(self) -> Option<Cool> { None }
}

fn woke(is_cool: impl Into<Option<Cool>>) {
    dbg!(is_cool.into().is_some());
}

fn main() {
    broke(false); // it it cool or not cool?
    woke(Cool);   // extremely clear!
    woke(!Cool);  // still clear!
}
