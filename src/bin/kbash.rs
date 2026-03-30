#![no_std]
#![no_main]

use knu::io;
use knu::proc;
use knu::syscalls;

knu::entry_point!();

const MAX_LINE: usize = 256;

unsafe fn read_line(buf: &mut [u8]) -> usize {
    let mut pos = 0;
    while pos < buf.len() - 1 {
        let mut ch = [0u8; 1];
        let n = knu::io::read_stdin(ch.as_mut_ptr(), 1);
        if n <= 0 { break; }
        if ch[0] == b'\n' { break; }
        buf[pos] = ch[0];
        pos += 1;
    }
    buf[pos] = 0;
    pos
}

unsafe fn parse_args(line: &mut [u8], argv: &mut [*const u8; 16]) -> usize {
    let mut argc = 0;
    let mut in_word = false;
    for i in 0..line.len() {
        if line[i] == 0 { break; }
        if line[i] == b' ' {
            line[i] = 0;
            in_word = false;
        } else if !in_word {
            if argc < 15 {
                argv[argc] = line.as_ptr().offset(i as isize);
                argc += 1;
            }
            in_word = true;
        }
    }
    argv[argc] = core::ptr::null();
    argc
}

unsafe fn streq(a: *const u8, b: &[u8]) -> bool {
    let mut i = 0;
    loop {
        let ca = *a.offset(i as isize);
        let cb = if i < b.len() { b[i] } else { 0 };
        if ca != cb { return false; }
        if ca == 0 { return true; }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn knu_main(_argc: isize, _argv: *const *const u8) -> ! {
    io::print("\x1b[38;5;213mKNU OS (v0.5.0) - 256 Colors Edition\x1b[0m\n");

    loop {
        let mut cwd = [0u8; 256];
        let len = unsafe {
            #[cfg(target_os = "linux")]
            let l = syscalls::getcwd(cwd.as_mut_ptr(), cwd.len());
            #[cfg(not(target_os = "linux"))]
            let l = syscalls::knu_sys_getcwd(cwd.as_mut_ptr(), cwd.len());
            l
        };

        io::print("\x1b[38;5;46mknu\x1b[0m:");
        io::print("\x1b[38;5;39m");
        if len > 0 {
            unsafe { knu::io::write_bytes(cwd.as_ptr(), (len - 1) as usize) };
        } else {
            io::print("/");
        }
        io::print("\x1b[0m# ");

        let mut line = [0u8; MAX_LINE];
        let l = unsafe { read_line(&mut line) };
        
        if l == 0 { continue; }

        let mut argv: [*const u8; 16] = [core::ptr::null(); 16];
        let argc = unsafe { parse_args(&mut line, &mut argv) };

        if argc == 0 { continue; }

        unsafe {
            if streq(argv[0], b"exit\0") {
                proc::exit(0);
            } else if streq(argv[0], b"kcd\0") {
                if argc > 1 {
                    #[cfg(target_os = "linux")]
                    syscalls::chdir(argv[1]);
                    #[cfg(not(target_os = "linux"))]
                    syscalls::knu_sys_chdir(argv[1]);
                } else {
                    io::print("Usage: kcd <directory>\n");
                }
            } else if streq(argv[0], b"kecho\0") {
                for i in 1..argc {
                    let mut a_len = 0;
                    while *argv[i].offset(a_len) != 0 { a_len += 1; }
                    knu::io::write_bytes(argv[i], a_len as usize);
                    io::print(" ");
                }
                io::print("\n");
            } else if streq(argv[0], b"kcat\0") {
                if argc > 1 {
                    #[cfg(target_os = "linux")]
                    let fd = syscalls::open(argv[1], 0, 0);
                    #[cfg(not(target_os = "linux"))]
                    let fd = syscalls::knu_sys_open(argv[1], 0, 0);

                    if fd >= 0 {
                        let mut buffer = [0u8; 4096];
                        loop {
                            #[cfg(target_os = "linux")]
                            let br = syscalls::read(fd as i32, buffer.as_mut_ptr(), 4096);
                            #[cfg(not(target_os = "linux"))]
                            let br = syscalls::knu_sys_read(fd as i32, buffer.as_mut_ptr(), 4096);
                            
                            if br <= 0 { break; }
                            knu::io::write_bytes(buffer.as_ptr(), br as usize);
                        }
                        #[cfg(target_os = "linux")]
                        syscalls::close(fd as i32);
                        #[cfg(not(target_os = "linux"))]
                        syscalls::knu_sys_close(fd as i32);
                    } else {
                        io::print("kcat: file not found\n");
                    }
                }
            } else if streq(argv[0], b"kls\0") {
                let target_dir = if argc > 1 { argv[1] } else { b".\0".as_ptr() };
                
                #[cfg(target_os = "linux")]
                let fd = syscalls::open(target_dir, 0, 0);
                #[cfg(not(target_os = "linux"))]
                let fd = syscalls::knu_sys_open(target_dir, 0, 0);

                if fd >= 0 {
                    let mut buf = [0u8; 4096];
                    loop {
                        #[cfg(target_os = "linux")]
                        let nread = syscalls::getdents64(fd as i32, buf.as_mut_ptr(), 4096);
                        #[cfg(not(target_os = "linux"))]
                        let nread = syscalls::knu_sys_getdents64(fd as i32, buf.as_mut_ptr(), 4096);

                        if nread <= 0 { break; }
                        let mut bpos = 0;
                        while bpos < nread as usize {
                            let d_reclen = (buf[bpos + 16] as u16) | ((buf[bpos + 17] as u16) << 8);
                            let d_type = buf[bpos + 18];
                            let name_ptr = buf.as_ptr().offset((bpos + 19) as isize);
                            let mut nlen = 0;
                            while *name_ptr.offset(nlen) != 0 { nlen += 1; }
                            
                            if !streq(name_ptr, b".\0") && !streq(name_ptr, b"..\0") {
                                if d_type == 4 {
                                    io::print("\x1b[38;5;39m");
                                } else if d_type == 10 {
                                    io::print("\x1b[38;5;51m");
                                } else if d_type == 2 || d_type == 6 {
                                    io::print("\x1b[38;5;220m");
                                } else {
                                    io::print("\x1b[38;5;253m");
                                }
                                knu::io::write_bytes(name_ptr, nlen as usize);
                                io::print("\x1b[0m  ");
                            }
                            bpos += d_reclen as usize;
                        }
                    }
                    io::print("\n");
                    #[cfg(target_os = "linux")]
                    syscalls::close(fd as i32);
                    #[cfg(not(target_os = "linux"))]
                    syscalls::knu_sys_close(fd as i32);
                } else {
                    io::print("kls: cannot open directory\n");
                }
            } else {
                let pid = proc::fork();
                if pid == 0 {
                    let envp: [*const u8; 1] = [core::ptr::null()];
                    proc::execve(argv[0], argv.as_ptr(), envp.as_ptr());
                    io::print("command not found\n");
                    proc::exit(1);
                } else if pid > 0 {
                    proc::waitpid(pid);
                }
            }
        }
    }
}
