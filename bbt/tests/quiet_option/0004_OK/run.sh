#!/usr/bin/env bash

biff -q -m 414141 file1.txt file2.txt 2>&1
echo $?
biff -q -m 414141 file2.txt file3.txt 2>&1
echo $?
biff -q -m 414141 file2.txt file4.txt 2>&1
echo $?
