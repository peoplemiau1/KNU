use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use crate::syscalls::{mmap, munmap};

pub struct KnuAllocator;

unsafe impl GlobalAlloc for KnuAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let addr = mmap(
            null_mut(),
            size,
            3,
            34,
            -1,
            0,
        );
        
        if addr as isize == -1 {
            null_mut()
        } else {
            addr as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        munmap(ptr, layout.size());
    }
}
