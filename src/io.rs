use crate::syscalls;

pub fn print(text: &str) {
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::write(1, text.as_ptr(), text.len());
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_write(1, text.as_ptr(), text.len());
    }
}

pub fn write_bytes(buf: *const u8, len: usize) {
    unsafe {
        #[cfg(target_os = "linux")]
        syscalls::write(1, buf, len);
        #[cfg(not(target_os = "linux"))]
        syscalls::knu_sys_write(1, buf, len);
    }
}

pub fn read_stdin(buf: &mut [u8]) -> isize {
    unsafe {
        #[cfg(target_os = "linux")]
        return syscalls::read(0, buf.as_mut_ptr(), buf.len());
        #[cfg(not(target_os = "linux"))]
        return syscalls::knu_sys_read(0, buf.as_mut_ptr(), buf.len());
    }
}
