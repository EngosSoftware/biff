#!/usr/bin/env bash

biff /proc/self/mem file.txt 2>&1
echo $?
biff file.txt /proc/self/mem 2>&1
echo $?
