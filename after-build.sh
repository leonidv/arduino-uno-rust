#!/usr/bin/env bash
# -*- coding: utf-8 -*-

printf "\n\n\033[1;34m========================================================================================\033[0m\n"
printf "\033[1;34m============================ AFTER BUILD SCRIPT ========================================\033[0m\n"

ELF=$1
HEX=$(echo $ELF | sed 's/.elf$/.hex/')
avr-objcopy -j .text -j .data -O ihex $ELF $HEX


printf "    \033[1;32mHEX File\033[0m \033[35m${HEX}\033[0m\n"

ravedude uno -cb 57600 $1