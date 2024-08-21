//thanks to https://t.me/terriblerust for this one!
use std::time::Duration;

fn f1<T>(v: &mut T) -> &mut T {
    println!("1");
    v
}

fn f2<T: Default>() -> T {
    println!("2");
    T::default()
}

fn main() {
    println!("evaluation order for non-primitive types:");
    let mut x = Duration::from_secs(2);
    *f1(&mut x) += f2::<Duration>();
    
    println!("evaluation order for primitive types:");
    let mut x = 0;
    *f1(&mut x) += f2::<i32>();
}
