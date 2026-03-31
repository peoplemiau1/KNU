#![no_std]
extern crate alloc;

pub mod allocator;
pub mod intrinsics;
pub mod panic;
pub mod error;
pub mod syscalls;
pub use syscalls as sys;
pub mod fs;
pub mod process;
pub mod env;
pub mod io;
pub mod libc;

#[macro_export]
macro_rules! entry_point {
    () => {
        core::arch::global_asm!(
            ".section .text",
            ".global _start",
            "_start:",
            "mov rdi, [rsp]",
            "lea rsi, [rsp + 8]",
            "mov rdx, rdi",
            "inc rdx",
            "shl rdx, 3",
            "add rdx, rsi",
            "and rsp, -16",
            "call knu_main",
            "mov rdi, 1",
            "mov rax, 60",
            "syscall"
        );
    };
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    process::exit(1);
}
