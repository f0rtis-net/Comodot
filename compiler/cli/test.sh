#!/bin/bash

cargo run
cd ./../internal/buildins
make all
cd ./../runtime
make all
cd ./../../cli
clang -o test buildins.bc runtime.bc include.o main.o
./test
echo $?
