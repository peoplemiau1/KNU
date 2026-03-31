#![no_std]
#![no_main]

extern crate alloc;
extern crate knu;

#[allow(unused_imports)]
use knu::allocator;

use knu::{io, process, env};

knu::entry_point!();

#[no_mangle]
pub extern "C" fn knu_main(argc: isize, argv: *const *const u8, _envp: *const *const u8) -> ! {
    let args = env::parse_args(argc, argv);
    
    if argc > 1 {
        let mut i = 1;
        while i < argc as usize && i < 32 {
            if !args[i].is_empty() {
                io::write_bytes(args[i].as_ptr(), args[i].len());
                if i < (argc as usize) - 1 {
                    io::print(" ");
                }
            }
            i += 1;
        }
    }
    io::print("\n");
    process::exit(0);
}
