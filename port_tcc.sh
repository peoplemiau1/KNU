#!/bin/bash
set -e
if [ ! -d "tinycc" ]; then
    git clone --depth 1 https://repo.or.cz/tinycc.git
fi
cd tinycc
./configure --prefix=$(pwd)/inst --with-libgcc=no --crtprefix=.. --libpaths=.. --includedir=../include
make -j$(nproc)
cp tcc ../releases/ktcc
cd ..
echo -e "\x1b[38;5;46m[+] TCC Ported: ./releases/ktcc\x1b[0m"
