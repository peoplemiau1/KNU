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
        if c == b'\n' || c == b'\r' { if pos > 0 { break; } else { continue; } }
        if c == 8 || c == 127 { if pos > 0 { pos -= 1; io::print("\x08 \x08"); } continue; }
        if c >= 32 && c <= 126 { buf[pos] = c; pos += 1; }
    }
    core::str::from_utf8(&buf[..pos]).unwrap_or("")
}
fn execute_command(cmd: &str, args: &[&str], envp: *const *const u8) {
    if cmd.starts_with("./") || cmd.starts_with('/') || cmd.contains('/') {
        process::exec(cmd, args, envp);
    } else {
        if let Some(path_var) = env::get_path(envp) {
            for dir in path_var.split(':') {
                let mut f_path = [0u8; 4096];
                let d_b = dir.as_bytes();
                let c_b = cmd.as_bytes();
                let mut idx = 0;
                for b in d_b { if idx < 4000 { f_path[idx] = *b; idx += 1; } }
                f_path[idx] = b'/'; idx += 1;
                for b in c_b { if idx < 4095 { f_path[idx] = *b; idx += 1; } }
                if let Ok(s) = core::str::from_utf8(&f_path[..idx]) { process::exec(s, args, envp); }
            }
        }
    }
    io::eprint("kbash: "); io::eprint(cmd); io::eprint(": command not found\n");
    process::exit(1);
}
#[no_mangle]
pub extern "C" fn knu_main(_argc: isize, _argv: *const *const u8, envp: *const *const u8) -> ! {
    io::print("\x1b[38;5;213mKNU OS - Shell v0.8\x1b[0m\n");
    loop {
        let mut c_buf = [0u8; 256];
        io::print("\x1b[38;5;46mknu\x1b[0m:\x1b[38;5;39m");
        if let Ok(l) = fs::getcwd(&mut c_buf) { if l > 1 { io::write_bytes(c_buf.as_ptr(), l - 1); } else { io::print("/"); } }
        io::print("\x1b[0m# ");
        let mut l_buf = [0u8; 256];
        let line = read_line_str(&mut l_buf);
        if line.is_empty() { continue; }
        let mut args = [""; 32];
        let mut argc = 0;
        for arg in line.split_whitespace() { if argc < 32 { args[argc] = arg; argc += 1; } }
        if argc == 0 { continue; }
        if args[0] == "exit" { process::exit(0); }
        let mut p_pos = None;
        for i in 0..argc { if args[i] == "|" { p_pos = Some(i); break; } }
        if let Some(pos) = p_pos {
            let l_args = &args[..pos];
            let r_args = &args[pos+1..argc];
            if let Ok(fds) = fs::pipe() {
                let pid1 = process::fork();
                if pid1 == 0 { fs::dup2(fds[1], 1); fs::close_fd(fds[0]); fs::close_fd(fds[1]); execute_command(l_args[0], l_args, envp); }
                let pid2 = process::fork();
                if pid2 == 0 { fs::dup2(fds[0], 0); fs::close_fd(fds[0]); fs::close_fd(fds[1]); execute_command(r_args[0], r_args, envp); }
                fs::close_fd(fds[0]); fs::close_fd(fds[1]);
                process::waitpid(pid1); process::waitpid(pid2);
            }
        } else {
            let mut r_out = None;
            let mut act_argc = argc;
            for i in 0..argc { if args[i] == ">" { if i+1 < argc { r_out = Some(args[i+1]); } act_argc = i; break; } }
            let c_args = &args[..act_argc];
            match c_args[0] {
                "kcd" => if act_argc > 1 { let _ = fs::chdir(c_args[1]); },
                cmd => {
                    let pid = process::fork();
                    if pid == 0 {
                        if let Some(f) = r_out { let _ = fs::redirect_out(f); }
                        execute_command(cmd, c_args, envp);
                    } else if pid > 0 { process::waitpid(pid); }
                }
            }
        }
    }
}
