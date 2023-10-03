#!/usr/bin/env bash

biff -q -a 2 file1.txt file2.txt 2>&1
echo $?
biff -s -a 2 file1.txt file2.txt 2>&1
echo $?
biff -q -a 50 file1.txt file2.txt 2>&1
echo $?
biff -q -p 10 file1.txt file2.txt 2>&1
echo $?
biff -s -p 10 file1.txt file2.txt 2>&1
echo $?
biff -q -p 90 file1.txt file2.txt 2>&1
echo $?
