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
    for i in 0..n { *s.offset(i as isize) = c as u8; }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n { *dest.offset(i as isize) = *src.offset(i as isize); }
    dest
}
