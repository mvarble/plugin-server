#!/bin/bash
gcc test.c -L/home/mvarble/dropbox/Dropbox/development/servers/rust/plugin-server/examples/c/ -ldl -lsolve -o test
./test
