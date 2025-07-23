#!/bin/bash
set -e

TARGET_DIR="./target/thumbv7em-none-eabihf/release"
NAME="bingo-fc"
ELF="$TARGET_DIR/$NAME"
BIN="$TARGET_DIR/$NAME.bin"
OBJCOPY="arm-none-eabi-objcopy"

cargo build --release

$OBJCOPY -O binary "$ELF" "$BIN"
dfu-util -a 0 -s 0x08000000:leave -D $BIN
