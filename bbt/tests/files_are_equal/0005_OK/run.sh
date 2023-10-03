#!/usr/bin/env bash

biff -i 6:5 file1.txt file2.txt 2>&1
echo $?