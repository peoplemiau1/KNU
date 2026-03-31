#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use knu::io;
use knu::proc;
use knu::syscalls;

knu::entry_point!();

const MAX_LINE: usize = 256;

fn read_line(buf: &mut [u8]) -> usize {
    let mut pos = 0;
    while pos < buf.len() - 1 {
        let mut ch = [0u8; 1];
        let n = io::read_stdin(&mut ch);
        if n <= 0 { break; }
        
        let c = ch[0];
        
        if c == b'\n' || c == b'\r' {
            if pos > 0 { break; } else { continue; }
        }
        
        if c == 8 || c == 127 {
            if pos > 0 {
                pos -= 1;
                io::print("\x08 \x08");
            }
            continue;
        }

        if c >= 32 && c <= 126 {
            buf[pos] = c;
            pos += 1;
        }
    }
    buf[pos] = 0;
    pos
}

fn parse_args(line: &mut [u8], argv: &mut [*const u8; 16]) -> usize {
    let mut argc = 0;
    let mut in_word = false;
    for i in 0..line.len() {
        if line[i] == 0 { break; }
        if line[i] == b' ' {
            line[i] = 0;
            in_word = false;
        } else if !in_word {
            if argc < 15 {
                argv[argc] = unsafe { line.as_ptr().offset(i as isize) };
                argc += 1;
            }
            in_word = true;
        }
    }
    argv[argc] = core::ptr::null();
    argc
}

fn streq(a: *const u8, b: &[u8]) -> bool {
    let mut i = 0;
    unsafe {
        loop {
            let ca = *a.offset(i as isize);
            let cb = if i < b.len() { b[i] } else { 0 };
            if ca != cb { return false; }
            if ca == 0 { return true; }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn knu_main(_argc: isize, _argv: *const *const u8, envp: *const *const u8) -> ! {
    io::print("\x1b[38;5;213mKNU OS (v0.7.2) - Safe API Edition\x1b[0m\n");

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
        if len > 1 {
            io::write_bytes(cwd.as_ptr(), (len - 1) as usize);
        } else {
            io::print("/");
        }
        io::print("\x1b[0m# ");

        let mut line = [0u8; MAX_LINE];
        let l = read_line(&mut line);
        
        if l == 0 { continue; }

        let mut argv: [*const u8; 16] = [core::ptr::null(); 16];
        let mut argc = parse_args(&mut line, &mut argv);

        if argc == 0 { continue; }

        let mut redirect_out: *const u8 = core::ptr::null();
        let mut clean_argc = 0;
        let mut j = 0;
        
        while j < argc {
            if streq(argv[j], b">\0") {
                if j + 1 < argc {
                    redirect_out = argv[j + 1];
                }
                break;
            }
            argv[clean_argc] = argv[j];
            clean_argc += 1;
            j += 1;
        }
        argv[clean_argc] = core::ptr::null();
        argc = clean_argc;

        if streq(argv[0], b"exit\0") {
            proc::exit(0);
        } else if streq(argv[0], b"kcd\0") {
            if argc > 1 {
                unsafe {
                    #[cfg(target_os = "linux")] syscalls::chdir(argv[1]);
                    #[cfg(not(target_os = "linux"))] syscalls::knu_sys_chdir(argv[1]);
                }
            }
        } else if streq(argv[0], b"krm\0") {
            if argc > 1 {
                unsafe {
                    #[cfg(target_os = "linux")] syscalls::unlink(argv[1]);
                    #[cfg(not(target_os = "linux"))] syscalls::knu_sys_unlink(argv[1]);
                }
            }
        } else if streq(argv[0], b"kmkdir\0") {
            if argc > 1 {
                unsafe {
                    #[cfg(target_os = "linux")] syscalls::mkdir(argv[1], 493);
                    #[cfg(not(target_os = "linux"))] syscalls::knu_sys_mkdir(argv[1], 493);
                }
            }
        } else if streq(argv[0], b"ktouch\0") {
            if argc > 1 {
                unsafe {
                    #[cfg(target_os = "linux")] {
                        let fd = syscalls::open(argv[1], 65, 420);
                        if fd >= 0 { syscalls::close(fd as i32); }
                    }
                    #[cfg(not(target_os = "linux"))] {
                        let fd = syscalls::knu_sys_open(argv[1], 65, 420);
                        if fd >= 0 { syscalls::knu_sys_close(fd as i32); }
                    }
                }
            }
        } else {
            let pid = proc::fork();
            if pid == 0 {
                unsafe {
                    if !redirect_out.is_null() {
                        #[cfg(target_os = "linux")] {
                            let fd = syscalls::open(redirect_out, 577, 420);
                            if fd >= 0 {
                                syscalls::dup2(fd as i32, 1);
                                syscalls::close(fd as i32);
                            }
                        }
                        #[cfg(not(target_os = "linux"))] {
                            let fd = syscalls::knu_sys_open(redirect_out, 577, 420);
                            if fd >= 0 {
                                syscalls::knu_sys_dup2(fd as i32, 1);
                                syscalls::knu_sys_close(fd as i32);
                            }
                        }
                    }

                    proc::execve(argv[0], argv.as_ptr(), envp);

                    let mut path_var: *const u8 = core::ptr::null();
                    let mut e = 0;
                    while !(*envp.offset(e)).is_null() {
                        let env_str = *envp.offset(e);
                        if *env_str.offset(0) == b'P' && *env_str.offset(1) == b'A' && 
                           *env_str.offset(2) == b'T' && *env_str.offset(3) == b'H' && *env_str.offset(4) == b'=' {
                            path_var = env_str.offset(5);
                            break;
                        }
                        e += 1;
                    }

                    if !path_var.is_null() {
                        let mut path_str = String::new();
                        let mut idx = 0;
                        loop {
                            let c = *path_var.offset(idx);
                            if c == 0 || c == b':' {
                                if !path_str.is_empty() {
                                    let mut full_path = path_str.clone();
                                    full_path.push('/');
                                    let mut cmd_idx = 0;
                                    while *argv[0].offset(cmd_idx) != 0 {
                                        full_path.push(*argv[0].offset(cmd_idx) as char);
                                        cmd_idx += 1;
                                    }
                                    full_path.push('\0');
                                    proc::execve(full_path.as_ptr(), argv.as_ptr(), envp);
                                }
                                path_str.clear();
                                if c == 0 { break; }
                            } else {
                                path_str.push(c as char);
                            }
                            idx += 1;
                        }
                    }
                }
                io::print("kbash: command not found\n");
                proc::exit(1);
            } else if pid > 0 {
                proc::waitpid(pid);
            }
        }
    }
}
