#!/usr/bin/env bash

biff -q file1.pdf file2.pdf 2>&1
biff -s file1.pdf file2.pdf 2>&1
biff --quiet file1.pdf file2.pdf 2>&1
biff --silent file1.pdf file2.pdf 2>&1
echo $?