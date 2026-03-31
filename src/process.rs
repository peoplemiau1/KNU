use crate::sys;
pub fn exit(code: i32) -> ! { unsafe { sys::exit(code) } }
pub fn fork() -> isize { unsafe { sys::fork() } }
pub fn waitpid(pid: isize) {
    let mut status: i32 = 0;
    unsafe { sys::wait4(pid, &mut status, 0, core::ptr::null_mut()) };
}
pub fn exec(path: &str, args: &[&str], envp: *const *const u8) -> isize {
    if path.len() >= 4096 { return -36; }
    let mut p_buf = [0u8; 4096];
    let p_b = path.as_bytes();
    for i in 0..p_b.len() { p_buf[i] = p_b[i]; }
    p_buf[p_b.len()] = 0;
    let mut argv_p: [*const u8; 32] = [core::ptr::null(); 32];
    let mut arg_b = [[0u8; 256]; 32];
    let mut i = 0;
    while i < args.len() && i < 31 {
        let a_b = args[i].as_bytes();
        let mut j = 0;
        while j < a_b.len() && j < 255 { arg_b[i][j] = a_b[j]; j += 1; }
        arg_b[i][j] = 0;
        argv_p[i] = arg_b[i].as_ptr();
        i += 1;
    }
    argv_p[i] = core::ptr::null();
    unsafe { sys::execve(p_buf.as_ptr(), argv_p.as_ptr(), envp) }
}
