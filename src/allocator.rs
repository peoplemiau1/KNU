use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct KnuAllocator;

unsafe impl GlobalAlloc for KnuAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        if size == 0 {
            return null_mut();
        }
        let addr = crate::syscalls::mmap(
            null_mut(),
            size,
            3,
            34,
            -1,
            0,
        );
        if addr == -1 {
            null_mut()
        } else {
            addr as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if !ptr.is_null() && layout.size() > 0 {
            crate::syscalls::munmap(ptr, layout.size());
        }
    }
}

#[global_allocator]
static ALLOCATOR: KnuAllocator = KnuAllocator;
