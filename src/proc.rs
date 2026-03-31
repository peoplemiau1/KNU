use crate::syscalls;
use alloc::string::String;

pub fn exit(code: i32) -> ! {
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::exit(code);
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_exit(code);
    }
}

pub fn get_pid() -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::getpid();
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_getpid();
    }
}

pub fn fork() -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::fork();
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_fork();
    }
}

pub fn waitpid(pid: isize) {
    let mut status: i32 = 0;
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::wait4(pid, &mut status, 0, core::ptr::null_mut());
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_wait4(pid, &mut status, 0, core::ptr::null_mut());
    }
}

pub fn execve(path: *const u8, args: *const *const u8, env: *const *const u8) -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::execve(path, args, env);
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_execve(path, args, env);
    }
}

pub fn exec(path: &str, args: &[*const u8], env: &[*const u8]) -> isize {
    let mut path_c = String::from(path);
    path_c.push('\0');
    execve(path_c.as_ptr(), args.as_ptr(), env.as_ptr())
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    exit(1);
}
