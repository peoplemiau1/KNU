pub use crate::intrinsics::memcpy;
use crate::sys;
use crate::error::SysError;
use alloc::string::String;

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_CREAT: i32 = 64;
pub const O_TRUNC: i32 = 512;

pub fn to_cstr(s: &str) -> String {
    let mut cstr = String::from(s);
    cstr.push('\0');
    cstr
}

pub fn open_file(path: &str, flags: i32, mode: i32) -> Result<i32, SysError> {
    if path.len() >= 4096 { return Err(SysError::NameTooLong); }
    let mut buf = [0u8; 4096];
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() { buf[i] = bytes[i]; i += 1; }
    buf[i] = 0;
    let fd = unsafe { sys::open(buf.as_ptr(), flags, mode) };
    if fd >= 0 { Ok(fd as i32) } else { Err(SysError::from_isize(fd)) }
}

pub fn read_fd(fd: i32, buf: &mut [u8]) -> Result<usize, SysError> {
    let n = unsafe { sys::read(fd, buf.as_mut_ptr(), buf.len()) };
    if n >= 0 { Ok(n as usize) } else { Err(SysError::from_isize(n)) }
}

pub fn close_fd(fd: i32) {
    unsafe { sys::close(fd) };
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize, SysError> {
    let res = unsafe { sys::getcwd(buf.as_mut_ptr(), buf.len()) };
    if res < 0 { Err(SysError::from_isize(res)) } else { Ok(res as usize) }
}

pub fn chdir(path: &str) -> Result<(), SysError> {
    let mut buf = [0u8; 4096];
    let b = path.as_bytes();
    let mut i = 0;
    while i < b.len() { buf[i] = b[i]; i += 1; }
    buf[i] = 0;
    let res = unsafe { sys::chdir(buf.as_ptr()) };
    if res < 0 { Err(SysError::from_isize(res)) } else { Ok(()) }
}

pub fn mkdir(path: &str, mode: i32) -> Result<(), SysError> {
    let mut buf = [0u8; 4096];
    let b = path.as_bytes();
    let mut i = 0;
    while i < b.len() { buf[i] = b[i]; i += 1; }
    buf[i] = 0;
    let res = unsafe { sys::mkdir(buf.as_ptr(), mode) };
    if res < 0 { Err(SysError::from_isize(res)) } else { Ok(()) }
}

pub fn unlink(path: &str) -> Result<(), SysError> {
    let mut buf = [0u8; 4096];
    let b = path.as_bytes();
    let mut i = 0;
    while i < b.len() { buf[i] = b[i]; i += 1; }
    buf[i] = 0;
    let res = unsafe { sys::unlink(buf.as_ptr()) };
    if res < 0 { Err(SysError::from_isize(res)) } else { Ok(()) }
}

pub fn touch(path: &str) -> Result<(), SysError> {
    match open_file(path, O_CREAT, 0o644) {
        Ok(fd) => { close_fd(fd); Ok(()) }
        Err(e) => Err(e),
    }
}

pub fn pipe() -> Result<[i32; 2], SysError> {
    let mut fds = [0i32; 2];
    let res = unsafe { sys::pipe(fds.as_mut_ptr()) };
    if res < 0 { Err(SysError::from_isize(res)) } else { Ok(fds) }
}

pub fn dup2(oldfd: i32, newfd: i32) {
    unsafe { sys::dup2(oldfd, newfd) };
}

pub fn redirect_out(path: &str) -> Result<(), SysError> {
    let flags = O_WRONLY | O_CREAT | O_TRUNC;
    match open_file(path, flags, 0o644) {
        Ok(fd) => { dup2(fd, 1); close_fd(fd); Ok(()) }
        Err(e) => Err(e),
    }
}
