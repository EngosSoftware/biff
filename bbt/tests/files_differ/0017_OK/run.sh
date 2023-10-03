#!/usr/bin/env bash

biff -l file1.txt file2.txt 2>&1
biff -l file2.txt file1.txt 2>&1
echo $?