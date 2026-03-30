use crate::syscalls;

pub fn exit(code: i32) -> ! {
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::exit(code);
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_exit(code);
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

pub fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::execve(path, argv, envp);
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_execve(path, argv, envp);
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

#[cfg(target_os = "linux")]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
