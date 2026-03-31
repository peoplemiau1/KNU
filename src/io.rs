use crate::syscalls;
pub fn write_bytes(buf: *const u8, len: usize) {
    unsafe { syscalls::write(1, buf, len); }
}
pub fn print(s: &str) {
    write_bytes(s.as_ptr(), s.len());
}
pub fn eprint(s: &str) {
    unsafe { syscalls::write(2, s.as_ptr(), s.len()); }
}
pub fn read_stdin(buf: &mut [u8]) -> isize {
    unsafe { syscalls::read(0, buf.as_mut_ptr(), buf.len()) }
}
