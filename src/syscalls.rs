#[cfg(target_os = "linux")]
pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 0isize => ret, in("rdi") fd, in("rsi") buf, in("rdx") count, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 1isize => ret, in("rdi") fd, in("rsi") buf, in("rdx") count, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn open(path: *const u8, flags: i32, mode: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 2isize => ret, in("rdi") path, in("rsi") flags, in("rdx") mode, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn close(fd: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 3isize => ret, in("rdi") fd, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn lseek(fd: i32, offset: isize, whence: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 8isize => ret, in("rdi") fd, in("rsi") offset, in("rdx") whence, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: isize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 9isize => ret, in("rdi") addr, in("rsi") len, in("rdx") prot, in("r10") flags, in("r8") fd, in("r9") off, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn mprotect(addr: *mut u8, len: usize, prot: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 10isize => ret, in("rdi") addr, in("rsi") len, in("rdx") prot, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn munmap(addr: *mut u8, len: usize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 11isize => ret, in("rdi") addr, in("rsi") len, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn brk(brk: *mut u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 12isize => ret, in("rdi") brk, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn pipe(pipefd: *mut i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 22isize => ret, in("rdi") pipefd, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn dup2(oldfd: i32, newfd: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 33isize => ret, in("rdi") oldfd, in("rsi") newfd, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn nanosleep(req: *const u8, rem: *mut u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 35isize => ret, in("rdi") req, in("rsi") rem, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn getpid() -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 39isize => ret, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn socket(domain: i32, type_: i32, protocol: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 41isize => ret, in("rdi") domain, in("rsi") type_, in("rdx") protocol, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn connect(fd: i32, addr: *const u8, len: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 42isize => ret, in("rdi") fd, in("rsi") addr, in("rdx") len, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn bind(fd: i32, addr: *const u8, len: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 49isize => ret, in("rdi") fd, in("rsi") addr, in("rdx") len, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn listen(fd: i32, backlog: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 50isize => ret, in("rdi") fd, in("rsi") backlog, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn accept(fd: i32, addr: *mut u8, len: *mut i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 43isize => ret, in("rdi") fd, in("rsi") addr, in("rdx") len, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn fork() -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 57isize => ret, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 59isize => ret, in("rdi") path, in("rsi") argv, in("rdx") envp, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn exit(code: i32) -> ! {
    core::arch::asm!("syscall", in("rax") 60, in("rdi") code, options(noreturn));
}

#[cfg(target_os = "linux")]
pub unsafe fn wait4(pid: isize, status: *mut i32, options: i32, rusage: *mut u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 61isize => ret, in("rdi") pid, in("rsi") status, in("rdx") options, in("r10") rusage, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn kill(pid: isize, sig: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 62isize => ret, in("rdi") pid, in("rsi") sig, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn getcwd(buf: *mut u8, size: usize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 79isize => ret, in("rdi") buf, in("rsi") size, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn chdir(path: *const u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 80isize => ret, in("rdi") path, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn mkdir(path: *const u8, mode: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 83isize => ret, in("rdi") path, in("rsi") mode, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn rmdir(path: *const u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 84isize => ret, in("rdi") path, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn unlink(path: *const u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 87isize => ret, in("rdi") path, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn chmod(path: *const u8, mode: i32) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 90isize => ret, in("rdi") path, in("rsi") mode, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn gettimeofday(tv: *mut u8, tz: *mut u8) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 96isize => ret, in("rdi") tv, in("rsi") tz, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(target_os = "linux")]
pub unsafe fn getdents64(fd: i32, dirp: *mut u8, count: usize) -> isize {
    let ret: isize;
    core::arch::asm!("syscall", inout("rax") 217isize => ret, in("rdi") fd, in("rsi") dirp, in("rdx") count, lateout("rcx") _, lateout("r11") _,);
    ret
}

#[cfg(not(target_os = "linux"))]
unsafe extern "C" {
    pub fn knu_sys_read(fd: i32, buf: *mut u8, count: usize) -> isize;
    pub fn knu_sys_write(fd: i32, buf: *const u8, count: usize) -> isize;
    pub fn knu_sys_open(path: *const u8, flags: i32, mode: i32) -> isize;
    pub fn knu_sys_close(fd: i32) -> isize;
    pub fn knu_sys_lseek(fd: i32, offset: isize, whence: i32) -> isize;
    pub fn knu_sys_mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: isize) -> isize;
    pub fn knu_sys_mprotect(addr: *mut u8, len: usize, prot: i32) -> isize;
    pub fn knu_sys_munmap(addr: *mut u8, len: usize) -> isize;
    pub fn knu_sys_brk(brk: *mut u8) -> isize;
    pub fn knu_sys_pipe(pipefd: *mut i32) -> isize;
    pub fn knu_sys_dup2(oldfd: i32, newfd: i32) -> isize;
    pub fn knu_sys_nanosleep(req: *const u8, rem: *mut u8) -> isize;
    pub fn knu_sys_getpid() -> isize;
    pub fn knu_sys_socket(domain: i32, type_: i32, protocol: i32) -> isize;
    pub fn knu_sys_connect(fd: i32, addr: *const u8, len: i32) -> isize;
    pub fn knu_sys_bind(fd: i32, addr: *const u8, len: i32) -> isize;
    pub fn knu_sys_listen(fd: i32, backlog: i32) -> isize;
    pub fn knu_sys_accept(fd: i32, addr: *mut u8, len: *mut i32) -> isize;
    pub fn knu_sys_fork() -> isize;
    pub fn knu_sys_execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize;
    pub fn knu_sys_exit(code: i32) -> !;
    pub fn knu_sys_wait4(pid: isize, status: *mut i32, options: i32, rusage: *mut u8) -> isize;
    pub fn knu_sys_kill(pid: isize, sig: i32) -> isize;
    pub fn knu_sys_getcwd(buf: *mut u8, size: usize) -> isize;
    pub fn knu_sys_chdir(path: *const u8) -> isize;
    pub fn knu_sys_mkdir(path: *const u8, mode: i32) -> isize;
    pub fn knu_sys_rmdir(path: *const u8) -> isize;
    pub fn knu_sys_unlink(path: *const u8) -> isize;
    pub fn knu_sys_chmod(path: *const u8, mode: i32) -> isize;
    pub fn knu_sys_gettimeofday(tv: *mut u8, tz: *mut u8) -> isize;
    pub fn knu_sys_getdents64(fd: i32, dirp: *mut u8, count: usize) -> isize;
}
