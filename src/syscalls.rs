#![allow(dead_code)]

macro_rules! s0 { ($n:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s1 { ($n:expr, $a:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s2 { ($n:expr, $a:expr, $b:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), in("rsi") ($b as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s3 { ($n:expr, $a:expr, $b:expr, $c:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), in("rsi") ($b as usize), in("rdx") ($c as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s4 { ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), in("rsi") ($b as usize), in("rdx") ($c as usize), in("r10") ($d as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s5 { ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), in("rsi") ($b as usize), in("rdx") ($c as usize), in("r10") ($d as usize), in("r8") ($e as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }
macro_rules! s6 { ($n:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => { { let mut r: usize = $n as usize; unsafe { core::arch::asm!("syscall", inout("rax") r, in("rdi") ($a as usize), in("rsi") ($b as usize), in("rdx") ($c as usize), in("r10") ($d as usize), in("r8") ($e as usize), in("r9") ($f as usize), lateout("rcx") _, lateout("r11") _); } r as isize } } }

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize { s3!(0, fd, buf, count) }
pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize { s3!(1, fd, buf, count) }
pub unsafe fn open(path: *const u8, flags: i32, mode: i32) -> isize { s3!(2, path, flags, mode) }
pub unsafe fn close(fd: i32) -> isize { s1!(3, fd) }
pub unsafe fn stat(path: *const u8, buf: *mut u8) -> isize { s2!(4, path, buf) }
pub unsafe fn fstat(fd: i32, buf: *mut u8) -> isize { s2!(5, fd, buf) }
pub unsafe fn lstat(path: *const u8, buf: *mut u8) -> isize { s2!(6, path, buf) }
pub unsafe fn poll(fds: *mut u8, nfds: usize, timeout: i32) -> isize { s3!(7, fds, nfds, timeout) }
pub unsafe fn lseek(fd: i32, offset: isize, whence: i32) -> isize { s3!(8, fd, offset, whence) }
pub unsafe fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: isize) -> isize { s6!(9, addr, len, prot, flags, fd, off) }
pub unsafe fn mprotect(addr: *mut u8, len: usize, prot: i32) -> isize { s3!(10, addr, len, prot) }
pub unsafe fn munmap(addr: *mut u8, len: usize) -> isize { s2!(11, addr, len) }
pub unsafe fn brk(brk: *mut u8) -> isize { s1!(12, brk) }
pub unsafe fn rt_sigaction(sig: i32, act: *const u8, oact: *mut u8, sigsetsize: usize) -> isize { s4!(13, sig, act, oact, sigsetsize) }
pub unsafe fn rt_sigprocmask(how: i32, set: *const u8, oset: *mut u8, sigsetsize: usize) -> isize { s4!(14, how, set, oset, sigsetsize) }
pub unsafe fn rt_sigreturn() -> ! { core::arch::asm!("syscall", in("rax") 15, options(noreturn)); }
pub unsafe fn ioctl(fd: i32, cmd: usize, arg: usize) -> isize { s3!(16, fd, cmd, arg) }
pub unsafe fn pread64(fd: i32, buf: *mut u8, count: usize, offset: isize) -> isize { s4!(17, fd, buf, count, offset) }
pub unsafe fn pwrite64(fd: i32, buf: *const u8, count: usize, offset: isize) -> isize { s4!(18, fd, buf, count, offset) }
pub unsafe fn readv(fd: i32, iov: *const u8, iovcnt: i32) -> isize { s3!(19, fd, iov, iovcnt) }
pub unsafe fn writev(fd: i32, iov: *const u8, iovcnt: i32) -> isize { s3!(20, fd, iov, iovcnt) }
pub unsafe fn access(path: *const u8, mode: i32) -> isize { s2!(21, path, mode) }
pub unsafe fn pipe(pipefd: *mut i32) -> isize { s1!(22, pipefd) }
pub unsafe fn select(n: i32, inp: *mut u8, outp: *mut u8, exp: *mut u8, tvp: *mut u8) -> isize { s5!(23, n, inp, outp, exp, tvp) }
pub unsafe fn sched_yield() -> isize { s0!(24) }
pub unsafe fn mremap(addr: *mut u8, old_len: usize, new_len: usize, flags: i32, new_addr: *mut u8) -> isize { s5!(25, addr, old_len, new_len, flags, new_addr) }
pub unsafe fn msync(addr: *mut u8, len: usize, flags: i32) -> isize { s3!(26, addr, len, flags) }
pub unsafe fn mincore(addr: *mut u8, len: usize, vec: *mut u8) -> isize { s3!(27, addr, len, vec) }
pub unsafe fn madvise(addr: *mut u8, len: usize, advice: i32) -> isize { s3!(28, addr, len, advice) }
pub unsafe fn shmget(key: i32, size: usize, shmflg: i32) -> isize { s3!(29, key, size, shmflg) }
pub unsafe fn shmat(shmid: i32, shmaddr: *const u8, shmflg: i32) -> isize { s3!(30, shmid, shmaddr, shmflg) }
pub unsafe fn shmctl(shmid: i32, cmd: i32, buf: *mut u8) -> isize { s3!(31, shmid, cmd, buf) }
pub unsafe fn dup(oldfd: i32) -> isize { s1!(32, oldfd) }
pub unsafe fn dup2(oldfd: i32, newfd: i32) -> isize { s2!(33, oldfd, newfd) }
pub unsafe fn pause() -> isize { s0!(34) }
pub unsafe fn nanosleep(req: *const u8, rem: *mut u8) -> isize { s2!(35, req, rem) }
pub unsafe fn getpid() -> isize { s0!(39) }
pub unsafe fn sendfile(out_fd: i32, in_fd: i32, offset: *mut isize, count: usize) -> isize { s4!(40, out_fd, in_fd, offset, count) }
pub unsafe fn socket(domain: i32, type_: i32, protocol: i32) -> isize { s3!(41, domain, type_, protocol) }
pub unsafe fn connect(fd: i32, addr: *const u8, len: i32) -> isize { s3!(42, fd, addr, len) }
pub unsafe fn accept(fd: i32, addr: *mut u8, len: *mut i32) -> isize { s3!(43, fd, addr, len) }
pub unsafe fn sendto(fd: i32, buf: *const u8, len: usize, flags: i32, addr: *const u8, addrlen: i32) -> isize { s6!(44, fd, buf, len, flags, addr, addrlen) }
pub unsafe fn recvfrom(fd: i32, buf: *mut u8, len: usize, flags: i32, addr: *mut u8, addrlen: *mut i32) -> isize { s6!(45, fd, buf, len, flags, addr, addrlen) }
pub unsafe fn sendmsg(fd: i32, msg: *const u8, flags: i32) -> isize { s3!(46, fd, msg, flags) }
pub unsafe fn recvmsg(fd: i32, msg: *mut u8, flags: i32) -> isize { s3!(47, fd, msg, flags) }
pub unsafe fn shutdown(fd: i32, how: i32) -> isize { s2!(48, fd, how) }
pub unsafe fn bind(fd: i32, addr: *const u8, len: i32) -> isize { s3!(49, fd, addr, len) }
pub unsafe fn listen(fd: i32, backlog: i32) -> isize { s2!(50, fd, backlog) }
pub unsafe fn getsockname(fd: i32, addr: *mut u8, len: *mut i32) -> isize { s3!(51, fd, addr, len) }
pub unsafe fn getpeername(fd: i32, addr: *mut u8, len: *mut i32) -> isize { s3!(52, fd, addr, len) }
pub unsafe fn socketpair(domain: i32, type_: i32, protocol: i32, sv: *mut i32) -> isize { s4!(53, domain, type_, protocol, sv) }
pub unsafe fn clone(flags: usize, child_stack: *mut u8, ptid: *mut i32, ctid: *mut i32, tls: *mut u8) -> isize { s5!(56, flags, child_stack, ptid, ctid, tls) }
pub unsafe fn fork() -> isize { s0!(57) }
pub unsafe fn vfork() -> isize { s0!(58) }
pub unsafe fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize { s3!(59, path, argv, envp) }
pub unsafe fn exit(code: i32) -> ! { core::arch::asm!("syscall", in("rax") 60, in("rdi") code as usize, options(noreturn)); }
pub unsafe fn wait4(pid: isize, status: *mut i32, options: i32, rusage: *mut u8) -> isize { s4!(61, pid, status, options, rusage) }
pub unsafe fn kill(pid: isize, sig: i32) -> isize { s2!(62, pid, sig) }
pub unsafe fn uname(buf: *mut u8) -> isize { s1!(63, buf) }
pub unsafe fn fcntl(fd: i32, cmd: i32, arg: usize) -> isize { s3!(72, fd, cmd, arg) }
pub unsafe fn flock(fd: i32, operation: i32) -> isize { s2!(73, fd, operation) }
pub unsafe fn fsync(fd: i32) -> isize { s1!(74, fd) }
pub unsafe fn fdatasync(fd: i32) -> isize { s1!(75, fd) }
pub unsafe fn truncate(path: *const u8, length: isize) -> isize { s2!(76, path, length) }
pub unsafe fn ftruncate(fd: i32, length: isize) -> isize { s2!(77, fd, length) }
pub unsafe fn getdents(fd: i32, dirp: *mut u8, count: usize) -> isize { s3!(78, fd, dirp, count) }
pub unsafe fn getcwd(buf: *mut u8, size: usize) -> isize { s2!(79, buf, size) }
pub unsafe fn chdir(path: *const u8) -> isize { s1!(80, path) }
pub unsafe fn fchdir(fd: i32) -> isize { s1!(81, fd) }
pub unsafe fn rename(oldname: *const u8, newname: *const u8) -> isize { s2!(82, oldname, newname) }
pub unsafe fn mkdir(path: *const u8, mode: i32) -> isize { s2!(83, path, mode) }
pub unsafe fn rmdir(path: *const u8) -> isize { s1!(84, path) }
pub unsafe fn creat(path: *const u8, mode: i32) -> isize { s2!(85, path, mode) }
pub unsafe fn link(oldname: *const u8, newname: *const u8) -> isize { s2!(86, oldname, newname) }
pub unsafe fn unlink(path: *const u8) -> isize { s1!(87, path) }
pub unsafe fn symlink(oldname: *const u8, newname: *const u8) -> isize { s2!(88, oldname, newname) }
pub unsafe fn readlink(path: *const u8, buf: *mut u8, bufsiz: usize) -> isize { s3!(89, path, buf, bufsiz) }
pub unsafe fn chmod(path: *const u8, mode: i32) -> isize { s2!(90, path, mode) }
pub unsafe fn fchmod(fd: i32, mode: i32) -> isize { s2!(91, fd, mode) }
pub unsafe fn chown(path: *const u8, owner: i32, group: i32) -> isize { s3!(92, path, owner, group) }
pub unsafe fn fchown(fd: i32, owner: i32, group: i32) -> isize { s3!(93, fd, owner, group) }
pub unsafe fn lchown(path: *const u8, owner: i32, group: i32) -> isize { s3!(94, path, owner, group) }
pub unsafe fn umask(mask: i32) -> isize { s1!(95, mask) }
pub unsafe fn gettimeofday(tv: *mut u8, tz: *mut u8) -> isize { s2!(96, tv, tz) }
pub unsafe fn getrlimit(resource: i32, rlim: *mut u8) -> isize { s2!(97, resource, rlim) }
pub unsafe fn getrusage(who: i32, usage: *mut u8) -> isize { s2!(98, who, usage) }
pub unsafe fn sysinfo(info: *mut u8) -> isize { s1!(99, info) }
pub unsafe fn getuid() -> isize { s0!(102) }
pub unsafe fn getgid() -> isize { s0!(104) }
pub unsafe fn geteuid() -> isize { s0!(107) }
pub unsafe fn getegid() -> isize { s0!(108) }
pub unsafe fn getppid() -> isize { s0!(110) }
pub unsafe fn getpgrp() -> isize { s0!(111) }
pub unsafe fn setsid() -> isize { s0!(112) }
pub unsafe fn getpgid(pid: i32) -> isize { s1!(121, pid) }
pub unsafe fn statfs(path: *const u8, buf: *mut u8) -> isize { s2!(137, path, buf) }
pub unsafe fn fstatfs(fd: i32, buf: *mut u8) -> isize { s2!(138, fd, buf) }
pub unsafe fn prctl(option: i32, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> isize { s5!(157, option, arg2, arg3, arg4, arg5) }
pub unsafe fn chroot(path: *const u8) -> isize { s1!(161, path) }
pub unsafe fn sync() -> isize { s0!(162) }
pub unsafe fn mount(source: *const u8, target: *const u8, filesystemtype: *const u8, mountflags: usize, data: *const u8) -> isize { s5!(165, source, target, filesystemtype, mountflags, data) }
pub unsafe fn umount2(target: *const u8, flags: i32) -> isize { s2!(166, target, flags) }
pub unsafe fn gettid() -> isize { s0!(186) }
pub unsafe fn futex(uaddr: *mut i32, futex_op: i32, val: i32, timeout: *const u8, uaddr2: *mut i32, val3: i32) -> isize { s6!(202, uaddr, futex_op, val, timeout, uaddr2, val3) }
pub unsafe fn epoll_create(size: i32) -> isize { s1!(213, size) }
pub unsafe fn getdents64(fd: i32, dirp: *mut u8, count: usize) -> isize { s3!(217, fd, dirp, count) }
pub unsafe fn set_tid_address(tidptr: *mut i32) -> isize { s1!(218, tidptr) }
pub unsafe fn clock_gettime(clk_id: i32, tp: *mut u8) -> isize { s2!(228, clk_id, tp) }
pub unsafe fn exit_group(code: i32) -> ! { core::arch::asm!("syscall", in("rax") 231, in("rdi") code as usize, options(noreturn)); }
pub unsafe fn epoll_wait(epfd: i32, events: *mut u8, maxevents: i32, timeout: i32) -> isize { s4!(232, epfd, events, maxevents, timeout) }
pub unsafe fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut u8) -> isize { s4!(233, epfd, op, fd, event) }
pub unsafe fn timerfd_create(clockid: i32, flags: i32) -> isize { s2!(283, clockid, flags) }
pub unsafe fn epoll_create1(flags: i32) -> isize { s1!(291, flags) }
