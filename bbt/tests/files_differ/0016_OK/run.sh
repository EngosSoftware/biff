#!/usr/bin/env bash

biff -l file1.txt file2.txt 2>&1
biff -lo file1.txt file2.txt 2>&1
biff -lx file1.txt file2.txt 2>&1
echo $?