#!/usr/bin/env bash

biff -q -p 0.7 -m 255044462d312e340a25 file1.pdf file2.pdf 2>&1
echo $?