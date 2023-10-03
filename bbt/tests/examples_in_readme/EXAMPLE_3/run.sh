#!/usr/bin/env bash

biff -a 3 sample1.txt sample2.txt
echo $?
biff -p 26.8 sample1.txt sample2.txt
echo $?
biff -q -p 26.8 sample1.txt sample2.txt
echo $?