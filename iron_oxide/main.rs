// Tiny "Hello, World!" program in Rust.
// build with rustc +nightly -C opt_level=3 main.rs
#![feature(no_core, lang_items)]
#![no_std]
#![no_core]
#![no_main]

#[lang = "panic_info"]
struct PanicInfo {}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"]
extern fn eh_personality() {}
#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
#[lang = "freeze"]
unsafe trait Freeze {}

#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
    fn exit(status: i32) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let s = b"Hello, World!\n";
    write(1, s as *const u8 as *const i8, 14);
    exit(0);
}
