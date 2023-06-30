#!/bin/bash

file="$1"

rustc $file -o solution
./solution
rm -f solution

