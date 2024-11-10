#!/bin/bash

cargo run
cd ./../internal/buildins
make all
cd ./../runtime
make all
cd ./../../cli
clang -o test buildins.bc runtime.bc test_unit.o
./test
echo $?
