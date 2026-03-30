#![no_std]
#![no_main]

use knu::syscalls;
use knu::proc;

#[no_mangle]
pub extern "C" fn knu_main(argc: isize, argv: *const *const u8) -> ! {
    unsafe {
        if argc > 1 {
            for i in 1..argc {
                let arg_ptr = *argv.offset(i as isize);
                let mut len = 0;
                while *arg_ptr.offset(len) != 0 {
                    len += 1;
                }
                
                #[cfg(target_os = "linux")]
                syscalls::write(1, arg_ptr, len as usize);
                
                #[cfg(not(target_os = "linux"))]
                syscalls::knu_sys_write(1, arg_ptr, len as usize);

                knu::io::print(" ");
            }
        }
        knu::io::print("\n");
    }
    proc::exit(0);
}
