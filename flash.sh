#!/bin/bash


TARGET_DIR="./target/thumbv7em-none-eabihf/release"
ELF_NAME="bingo-fc"
HEX_NAME="$ELF_NAME.hex"
DFU_NAME="$ELF_NAME.dfu"

cargo build --release

objcopy -O ihex "$TARGET_DIR/$ELF_NAME" "$TARGET_DIR/$HEX_NAME"
hex2dfu -i "$TARGET_DIR/$HEX_NAME" -o "$TARGET_DIR/$DFU_NAME"
dfu-util -D "$TARGET_DIR/$DFU_NAME"
