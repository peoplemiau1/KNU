#ifndef KNU_KLIBC_H
#define KNU_KLIBC_H
typedef long isize;
typedef unsigned long usize;
typedef isize ssize_t;
typedef usize size_t;
#define NULL ((void*)0)
extern void* malloc(size_t size);
extern void free(void* ptr);
extern int printf(const char* fmt);
extern void exit(int code);
extern ssize_t write(int fd, const void* buf, size_t count);
extern ssize_t read(int fd, void* buf, size_t count);
#endif
