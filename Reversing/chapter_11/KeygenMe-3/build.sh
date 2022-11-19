#!/bin/sh

nasm -felf keygen.asm && gcc -m32 keygen.o -o keygen
