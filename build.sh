#!/bin/bash

set -e

# Цвета для вывода
GREEN='\033[1;32m'
BLUE='\033[1;34m'
RED='\033[1;31m'
NC='\033[0m'

TARGET_DIR="$HOME/.cargo-target/x86_64-unknown-linux-gnu/release"

echo -e "${BLUE}[*] Начинаем сборку KNU OS...${NC}"

echo -e "${BLUE}[*] 1/4 Компиляция динамической библиотеки (libknu.so)...${NC}"
cargo build --lib --target x86_64-unknown-linux-gnu --release

echo -e "${BLUE}[*] 2/4 Компиляция оболочки (kbash)...${NC}"
cargo build --bin kbash --target x86_64-unknown-linux-gnu --release

echo -e "${BLUE}[*] 3/4 Очистка бинарников (strip) для минимального размера...${NC}"
if [ -f "$TARGET_DIR/libknu.so" ]; then
    strip "$TARGET_DIR/libknu.so"
fi
if [ -f "$TARGET_DIR/kbash" ]; then
    strip "$TARGET_DIR/kbash"
fi

echo -e "${BLUE}[*] 4/4 Копирование в папку releases/...${NC}"
mkdir -p releases
cp "$TARGET_DIR/libknu.so" releases/
cp "$TARGET_DIR/kbash" releases/

echo -e "${GREEN}[+] Сборка успешно завершена!${NC}"
ls -lh releases/

echo -e "\n${BLUE}[*] Запуск Python-теста библиотеки...${NC}"
if [ -f "test.py" ]; then
    python3 test.py
else
    echo -e "${RED}[-] Файл test.py не найден, пропускаем.${NC}"
fi

echo -e "\n${GREEN}[====== ГОТОВО ======]${NC}"
echo -e "Чтобы запустить KNU Bash, введи: ${BLUE}./releases/kbash${NC}"
