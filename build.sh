#!/bin/bash
set -e
export CARGO_TARGET_DIR="./target"
echo -e "\x1b[38;5;213m[*] Building KNU OS...\x1b[0m"
cargo build --release --lib
cargo build --release --bins
mkdir -p releases
cp ./target/release/libknu.so releases/
cp ./target/release/kbash releases/
cp ./target/release/kcat releases/
cp ./target/release/kecho releases/
echo -e "\x1b[38;5;213m[*] Building C-ABI Test...\x1b[0m"
gcc -c src/crt0.s -o releases/crt0.o
gcc -ffreestanding -fno-builtin -nostdlib -c src/coreutils/test_libc.c -o releases/test_libc.o
# Используем gcc для линковки - он сам найдет символы в .so
gcc -nostdlib -o releases/ktest_c releases/crt0.o releases/test_libc.o ./releases/libknu.so -Wl,-rpath,'$ORIGIN'
echo -e "\x1b[38;5;46m[+] Done!\x1b[0m"
