#!/bin/bash
set -e
TARGET_DIR="./target"
echo -e "\x1b[38;5;213m[*] Building KNU OS Library and Binaries...\x1b[0m"
cargo build --release --lib --target-dir $TARGET_DIR
cargo build --release --bins --target-dir $TARGET_DIR
mkdir -p releases
cp $TARGET_DIR/release/libknu.so releases/
cp $TARGET_DIR/release/kbash releases/
cp $TARGET_DIR/release/kcat releases/
cp $TARGET_DIR/release/kecho releases/
echo -e "\x1b[38;5;213m[*] Building C-ABI Bridge test...\x1b[0m"
gcc -c src/crt0.s -o releases/crt0.o
gcc -ffreestanding -fno-builtin -nostdlib -c src/coreutils/test_libc.c -o releases/test_libc.o
ld -nostdlib -dynamic-linker /lib64/ld-linux-x86-64.so.2 -rpath '$ORIGIN' releases/crt0.o releases/test_libc.o releases/libknu.so -o releases/ktest_c
echo -e "\x1b[38;5;46m[+] Done! Run: ./releases/kbash\x1b[0m"
echo -e "\x1b[38;5;46m[+] Test C code: ./releases/ktest_c\x1b[0m"
