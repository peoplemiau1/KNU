#![no_std]
#![no_main]

use knu::syscalls;
use knu::proc;
use knu::io;

const BUF_SIZE: usize = 4096;

#[no_mangle]
pub extern "C" fn knu_main(argc: isize, argv: *const *const u8) -> ! {
    if argc < 2 {
        io::print("Usage: kcat <file>\n");
        proc::exit(1);
    }

    unsafe {
        let path_ptr = *argv.offset(1);
        let fd = syscalls::open(path_ptr, 0, 0);
        
        if fd < 0 {
            io::print("Error\n");
            proc::exit(1);
        }

        let mut buffer = [0u8; BUF_SIZE];
        
        loop {
            let bytes_read = syscalls::read(fd as i32, buffer.as_mut_ptr(), BUF_SIZE);
            if bytes_read <= 0 {
                break;
            }
            
            #[cfg(target_os = "linux")]
            syscalls::write(1, buffer.as_ptr(), bytes_read as usize);
            
            #[cfg(not(target_os = "linux"))]
            syscalls::knu_sys_write(1, buffer.as_ptr(), bytes_read as usize);
        }

        syscalls::close(fd as i32);
    }

    proc::exit(0);
}
