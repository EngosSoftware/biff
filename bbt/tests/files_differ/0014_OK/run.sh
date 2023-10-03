#!/usr/bin/env bash

biff -b -d file1.txt file2.txt 2>&1
biff -b --decimal file1.txt file2.txt 2>&1
biff -b -o file1.txt file2.txt 2>&1
biff -b --octal file1.txt file2.txt 2>&1
biff -b -x file1.txt file2.txt 2>&1
biff -b --hexadecimal file1.txt file2.txt 2>&1
echo $?