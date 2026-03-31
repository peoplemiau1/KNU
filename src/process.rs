use crate::sys;

pub fn exit(code: i32) -> ! {
    unsafe { sys::exit(code) }
}

pub fn fork() -> isize {
    unsafe { sys::fork() }
}

pub fn waitpid(pid: isize) {
    let mut status: i32 = 0;
    unsafe { sys::wait4(pid, &mut status, 0, core::ptr::null_mut()) };
}

pub fn exec(path: &str, args: &[&str], envp: *const *const u8) -> isize {
    if path.len() >= 4096 { return -36; }
    let mut path_buf = [0u8; 4096];
    let p_bytes = path.as_bytes();
    let mut i = 0;
    while i < p_bytes.len() { path_buf[i] = p_bytes[i]; i += 1; }
    path_buf[i] = 0;

    let mut argv_ptrs: [*const u8; 32] = [core::ptr::null(); 32];
    let mut arg_bufs = [[0u8; 256]; 31];
    
    let mut arg_idx = 0;
    while arg_idx < args.len() && arg_idx < 31 {
        let a_bytes = args[arg_idx].as_bytes();
        let mut j = 0;
        while j < a_bytes.len() && j < 255 {
            arg_bufs[arg_idx][j] = a_bytes[j];
            j += 1;
        }
        arg_bufs[arg_idx][j] = 0;
        argv_ptrs[arg_idx] = arg_bufs[arg_idx].as_ptr();
        arg_idx += 1;
    }
    argv_ptrs[arg_idx] = core::ptr::null();

    unsafe { sys::execve(path_buf.as_ptr(), argv_ptrs.as_ptr(), envp) }
}
