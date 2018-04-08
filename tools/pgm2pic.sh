#!/bin/bash

if [ $# -eq 0 ]; then
    echo "usage pgm2pic.sh FILENAME.pgm"
    exit 1
fi

FILE=$1
if [[ $FILE == *.pgm ]]; then
    FILE=${FILE%.pgm}
fi

# if we could use L8:
# tail $FILE.pgm -n +5 > $FILE.pic

# as we use AL88, we have to add ff before every one:
tail $FILE.pgm -n +5 > tmp.l8
python3 l2al.py
mv tmp.al88 $FILE.pic
rm tmp.l8

