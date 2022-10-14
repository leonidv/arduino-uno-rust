#!/usr/bin/env bash
set -euf -o pipefail

PROJECT_FOLDER=$1

cd $PROJECT_FOLDER
cargo build -Z build-std=core --release --target avr-specs/avr-atmega328p.json

ELF=$(find target/avr-atmega328p/release/ -maxdepth 1  -name '*.elf')
HEX=$(echo $ELF | sed 's/.elf$/.hex/')

avr-objcopy -j .text -j .data -O ihex $ELF $HEX

cd - > /dev/null

printf "    \033[1;32mHEX File\033[0m \033[35m${HEX}\033[0m\n"
# avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:$ELF:e



