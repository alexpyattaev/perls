// thanks to https://t.me/terriblerust for this one!
// in rust, even unintialized memory initialized YOU!
#[allow(deprecated, invalid_value)]
fn main() {
    for _ in 0..1000 {
        let a: usize = unsafe { std::mem::uninitialized() };
        assert_eq!(a, 0x101010101010101);
    }
}
