#!/usr/bin/env bash

biff -a 7 file1.pdf file2.pdf 2>&1
echo $?