#![no_std]
#![no_main]

extern crate alloc;
extern crate knu;

#[allow(unused_imports)]
use knu::allocator;

use knu::{fs, io, process, env};

knu::entry_point!();

fn read_line_str(buf: &mut [u8]) -> &str {
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
            if pos > 0 { pos -= 1; io::print("\x08 \x08"); }
            continue;
        }
        if c >= 32 && c <= 126 { buf[pos] = c; pos += 1; }
    }
    core::str::from_utf8(&buf[..pos]).unwrap_or("")
}

#[no_mangle]
pub extern "C" fn knu_main(_argc: isize, _argv: *const *const u8, envp: *const *const u8) -> ! {
    io::print("\x1b[38;5;213mKNU OS - Zero Alloc Architecture\x1b[0m\n");

    loop {
        let mut cwd_buf = [0u8; 256];
        io::print("\x1b[38;5;46mknu\x1b[0m:\x1b[38;5;39m");
        if let Ok(len) = fs::getcwd(&mut cwd_buf) {
            if len > 1 {
                io::write_bytes(cwd_buf.as_ptr(), len - 1);
            } else {
                io::print("/");
            }
        }
        io::print("\x1b[0m# ");

        let mut line_buf = [0u8; 256];
        let line = read_line_str(&mut line_buf);
        if line.is_empty() { continue; }

        let mut args = [""; 32];
        let mut argc = 0;
        for arg in line.split_whitespace() {
            if argc < 32 {
                args[argc] = arg;
                argc += 1;
            }
        }
        if argc == 0 { continue; }

        let mut redirect_out = None;
        let mut i = 0;
        while i < argc {
            if args[i] == ">" {
                if i + 1 < argc {
                    redirect_out = Some(args[i + 1]);
                }
                argc = i;
                break;
            }
            i += 1;
        }
        if argc == 0 { continue; }

        let cmd_args = &args[..argc];

        match cmd_args[0] {
            "exit" => process::exit(0),
            "kcd" => if argc > 1 { 
                if let Err(e) = fs::chdir(cmd_args[1]) {
                    io::print(e.as_str()); io::print("\n");
                }
            },
            "krm" => if argc > 1 { 
                if let Err(e) = fs::unlink(cmd_args[1]) {
                    io::print(e.as_str()); io::print("\n");
                }
            },
            "kmkdir" => if argc > 1 { 
                if let Err(e) = fs::mkdir(cmd_args[1], 0o755) {
                    io::print(e.as_str()); io::print("\n");
                }
            },
            "ktouch" => if argc > 1 { 
                if let Err(e) = fs::touch(cmd_args[1]) {
                    io::print(e.as_str()); io::print("\n");
                }
            },
            cmd => {
                let pid = process::fork();
                if pid == 0 {
                    if let Some(file) = redirect_out {
                        let _ = fs::redirect_out(file);
                    }
                    process::exec(cmd, cmd_args, envp);
                    if let Some(path_var) = env::get_path(envp) {
                        for dir in path_var.split(':') {
                            let mut full_path = [0u8; 4096];
                            let d_bytes = dir.as_bytes();
                            let c_bytes = cmd.as_bytes();
                            let mut idx = 0;
                            
                            while idx < d_bytes.len() && idx < 4000 {
                                full_path[idx] = d_bytes[idx];
                                idx += 1;
                            }
                            full_path[idx] = b'/';
                            idx += 1;
                            
                            let mut c_idx = 0;
                            while c_idx < c_bytes.len() && idx < 4095 {
                                full_path[idx] = c_bytes[c_idx];
                                idx += 1;
                                c_idx += 1;
                            }
                            
                            if let Ok(s) = core::str::from_utf8(&full_path[..idx]) {
                                process::exec(s, cmd_args, envp);
                            }
                        }
                    }
                    io::print("kbash: command not found\n");
                    process::exit(1);
                } else if pid > 0 {
                    process::waitpid(pid);
                }
            }
        }
    }
}
