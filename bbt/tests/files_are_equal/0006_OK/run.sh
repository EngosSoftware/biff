#!/usr/bin/env bash

biff -i 6:5 -n 6 file1.txt file2.txt 2>&1
echo $?
biff --ignore-initial 6:5 --bytes 6 file1.txt file2.txt 2>&1
echo $?