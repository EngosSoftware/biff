#!/usr/bin/env bash

biff sample1.txt sample2.txt
echo $?
biff -b sample1.txt sample2.txt
echo $?
biff -bx sample1.txt sample2.txt
echo $?
biff -q sample1.txt sample2.txt
echo $?
biff -l sample1.txt sample2.txt
echo $?
biff -p 0.1 sample1.txt sample2.txt
echo $?
biff -a 1 sample1.txt sample2.txt
echo $?