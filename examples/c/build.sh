#!/bin/bash
# copied from https://stackoverflow.com/questions/14884126/build-so-file-from-c-file-using-gcc-command-line
gcc -Wall -g -shared -o libsolve.so -fPIC src.c
