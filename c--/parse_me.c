// Can you guess what this is????
unsigned long const volatile int  *(**bool[42]) ( long const volatile signed long *);
// Spoilers below!

















































//this is an array named "bool" made of 42 pointers to a function pointer, that each takes a const pointer to signed 64-bit integer, and returns a pointer to unsigned 32bit integer


/* automatically generated by rust-bindgen 0.69.4 */
/*
extern "C" {
    #[link_name = "\u{1}bool"]
    pub static mut bool_: [*mut ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_longlong,
        ) -> *const ::std::os::raw::c_ulong,
    >; 42usize];
}
*/