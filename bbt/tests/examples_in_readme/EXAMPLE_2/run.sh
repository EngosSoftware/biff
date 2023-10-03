#!/usr/bin/env bash

biff -i 2 sample1.txt sample2.txt 2>&1
echo $?