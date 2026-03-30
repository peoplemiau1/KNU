use crate::syscalls;

pub fn open(path: *const u8) -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::open(path, 0, 0);
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_open(path, 0, 0);
    }
}

pub fn read(fd: isize, buf: *mut u8, count: usize) -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::read(fd as i32, buf, count);
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_read(fd as i32, buf, count);
    }
}

pub fn close(fd: isize) {
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::close(fd as i32);
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_close(fd as i32);
    }
}
