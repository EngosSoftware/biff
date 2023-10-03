#!/usr/bin/env bash

biff file1.txt file2.txt 2>&1
echo $?
biff -q file1.txt file2.txt 2>&1
echo $?
biff -s file1.txt file2.txt 2>&1
echo $?