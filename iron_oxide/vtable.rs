#![feature(ptr_metadata)]
// thanks to https://t.me/terriblerust for this one!
// You can have vtables in rust, noone can stop you...

use std::{mem::transmute, ptr::{self, NonNull}};

// No implementors
trait Funny {
    fn method(&self);
}

#[repr(C)]
struct FunnyVtable {
    drop: unsafe fn(*mut ()),
    size: usize,
    alignment: usize,
    method: fn(*const ()),
}

unsafe fn noop_drop(_: *mut ()) {
    println!("Drop called!");
}

fn method(_: *const ()) {
    println!("Crimes!");
}

fn main() { unsafe {
    static VTABLE: FunnyVtable = FunnyVtable {
        drop: noop_drop,
        size: 0,
        alignment: 1,
        method: method,
    };
    
    let ptr: *mut dyn Funny = ptr::from_raw_parts_mut(
        NonNull::<()>::dangling().as_ptr(),
        transmute(&VTABLE),
    );
    let boxed = Box::from_raw(ptr);
    
    boxed.method();
} }
