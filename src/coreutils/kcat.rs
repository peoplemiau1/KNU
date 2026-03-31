#![no_std]
#![no_main]

extern crate alloc;
extern crate knu;

#[allow(unused_imports)]
use knu::allocator;

use knu::{io, process, fs, env};

knu::entry_point!();

#[no_mangle]
pub extern "C" fn knu_main(argc: isize, argv: *const *const u8, _envp: *const *const u8) -> ! {
    let args = env::parse_args(argc, argv);
    
    if argc < 2 || args[1].is_empty() {
        io::print("Usage: kcat <file>\n");
        process::exit(1);
    }

    match fs::open_file(args[1], fs::O_RDONLY, 0) {
        Ok(fd) => {
            let mut buf = [0u8; 1024];
            loop {
                match fs::read_fd(fd, &mut buf) {
                    Ok(0) => break,
                    Ok(n) => { io::write_bytes(buf.as_ptr(), n); },
                    Err(_) => break,
                }
            }
            fs::close_fd(fd);
        }
        Err(e) => {
            io::print("kcat: ");
            io::print(e.as_str());
            io::print("\n");
            process::exit(1);
        }
    }
    process::exit(0);
}
