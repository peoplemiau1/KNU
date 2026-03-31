#![no_std]

extern crate alloc;

pub mod syscalls;
pub mod io;
pub mod proc;
pub mod string;
pub mod panic;
pub mod fs;
pub mod allocator;

#[global_allocator]
static ALLOCATOR: allocator::KnuAllocator = allocator::KnuAllocator;

#[macro_export]
macro_rules! entry_point {
    () => {
        #[cfg(target_os = "linux")]
        core::arch::global_asm!(
            ".global _start",
            "_start:",
            "mov rdi, [rsp]",
            "lea rsi, [rsp + 8]",
            "lea rdx, [rsi + rdi * 8 + 8]",
            "and rsp, -16",
            "call knu_main",
        );
    };
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub extern "C" fn knu_hello() {
    io::print("Hello from KNU Shared Library (no_std, direct syscalls)!\n");
}
