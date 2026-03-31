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
        #[no_mangle]
        pub extern "C" fn _start(argc: isize, argv: *const *const u8, envp: *const *const u8) -> ! {
            knu_main(argc, argv, envp)
        }
    };
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    process::exit(1);
}
