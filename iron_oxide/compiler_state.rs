// thanks to https://t.me/terriblerust for this one!
trait Trait {
    fn abs(self) -> Self;
}

impl Trait for i64 {
    fn abs(self) -> Self {
        self * 2
    }
}

fn main() {
    // currently, type of `x` is `{integer}`:
    // the “polymorphic” type of all integer literals.
    let x = 42; 
    // we try to find the `.abs()` method.
    // there're no inherent methods yet
    // (because we don't know the exact type),
    // but `Trait::abs()` is available, so we call it.
    let a = x.abs();  // 84
    // now we can infer the type of `x` to be `i64`:
    // that's the only type that `Trait` is implemented for.
    // `i64` has an inherent method `.abs()`, so we call it.
    let b = x.abs();  // 42
    assert_eq!(a, b); // panics: 84 ≠ 42
}
