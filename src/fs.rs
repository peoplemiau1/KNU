use crate::syscalls;
use alloc::string::String;

pub struct File(i32);

impl File {
    pub fn open(path: &str, flags: i32, mode: i32) -> Option<Self> {
        let mut path_c = String::from(path);
        path_c.push('\0');
        unsafe {
            let fd = syscalls::open(path_c.as_ptr(), flags, mode);
            if fd >= 0 { Some(File(fd as i32)) } else { None }
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> isize {
        unsafe { syscalls::read(self.0, buf.as_mut_ptr(), buf.len()) }
    }

    pub fn write(&self, buf: &[u8]) -> isize {
        unsafe { syscalls::write(self.0, buf.as_ptr(), buf.len()) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { syscalls::close(self.0); }
    }
}

pub fn mkdir(path: &str) -> bool {
    let mut path_c = String::from(path);
    path_c.push('\0');
    unsafe { syscalls::mkdir(path_c.as_ptr(), 0o755) == 0 }
}

pub fn remove(path: &str) -> bool {
    let mut path_c = String::from(path);
    path_c.push('\0');
    unsafe { syscalls::unlink(path_c.as_ptr()) == 0 }
}
