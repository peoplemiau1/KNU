use crate::{io, process, allocator, fs};
use core::alloc::{Layout, GlobalAlloc};

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    if size == 0 { return core::ptr::null_mut(); }
    let layout = Layout::from_size_align(size + 8, 8).unwrap();
    let ptr = allocator::KnuAllocator.alloc(layout);
    if ptr.is_null() { return core::ptr::null_mut(); }
    *(ptr as *mut usize) = size;
    ptr.add(8)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() { return; }
    let real_ptr = ptr.sub(8);
    let size = *(real_ptr as *mut usize);
    let layout = Layout::from_size_align(size + 8, 8).unwrap();
    allocator::KnuAllocator.dealloc(real_ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn printf(fmt: *const u8) -> i32 {
    let mut len = 0;
    while *fmt.add(len) != 0 { len += 1; }
    io::write_bytes(fmt, len);
    len as i32
}

#[no_mangle]
pub unsafe extern "C" fn exit(code: i32) -> ! {
    process::exit(code);
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    while *s1.add(i) != 0 && *s1.add(i) == *s2.add(i) { i += 1; }
    (*s1.add(i) as i32) - (*s2.add(i) as i32)
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(d: *mut u8, s: *const u8) -> *mut u8 {
    let mut i = 0;
    while *s.add(i) != 0 { *d.add(i) = *s.add(i); i += 1; }
    *d.add(i) = 0;
    d
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const u8, c: i32) -> *mut u8 {
    let mut res = core::ptr::null_mut();
    let mut i = 0;
    loop {
        if *s.add(i) == c as u8 { res = s.add(i) as *mut u8; }
        if *s.add(i) == 0 { break; }
        i += 1;
    }
    res
}

#[no_mangle]
pub unsafe extern "C" fn fopen(path: *const u8, _mode: *const u8) -> *mut u8 {
    let mut len = 0;
    while *path.add(len) != 0 { len += 1; }
    let slice = core::slice::from_raw_parts(path, len);
    let s = core::str::from_utf8(slice).unwrap_or("");
    match fs::open_file(s, fs::O_RDONLY, 0) {
        Ok(fd) => fd as *mut u8,
        Err(_) => core::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn fclose(fp: *mut u8) -> i32 {
    fs::close_fd(fp as i32);
    0
}
