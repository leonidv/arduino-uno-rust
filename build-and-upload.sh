#!/usr/bin/env bash
set -euf -o pipefail

PROJECT_FOLDER=$1

cd $PROJECT_FOLDER
cargo build -Z build-std=core --release --target avr-specs/avr-atmega328p.json

ELF=$(find target/avr-atmega328p/release/ -maxdepth 1  -name '*.elf')
avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:$ELF:e

cd -
