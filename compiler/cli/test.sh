#!/bin/bash

cargo run
clang -o test gen.o
./test