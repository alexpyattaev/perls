//rustc -C opt_level=3 safe_segfault.rs

#[inline(never)]
fn print_vals(x: f64, i: usize, vals_i: u32) {
    println!("x={x} i={i} vals[i]={vals_i}");
}

#[inline(never)]
pub fn evil(vals: &[u32; 300]) {
    const INC: f64 = f64::MAX / 90.0;
    let mut x: f64 = -1.0;
    let mut i: usize = 0;

    while x.is_sign_negative() {
        print_vals(x, i, vals[i]);
        // Use a big decrement to reach negative infinity quickly.
        x -= INC;
        // Convert infinity into a NaN (Inf - Inf = NaN)
        x = x + x - x;
        i += 2;
    }
}

pub fn main() {
    let mut vals: [u32; 300] = [0; 300];
    for i in 0..300 { vals[i as usize] = i; }
    evil(&vals);
}
