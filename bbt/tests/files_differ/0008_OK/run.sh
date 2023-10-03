#!/usr/bin/env bash

biff -p 0.01 file1.pdf file2.pdf 2>&1
echo $?