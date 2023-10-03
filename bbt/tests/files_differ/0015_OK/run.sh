#!/usr/bin/env bash

biff --print-bytes file1.txt file2.txt 2>&1
biff --print-bytes file2.txt file1.txt 2>&1
echo $?