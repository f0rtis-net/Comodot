#!/bin/bash

cargo run
clang -emit-llvm -c -o buildins.bc builtins.c
clang -o test buildins.bc gen.o
./test