#!/bin/bash

FILE1=file1.avi
FILE2=file2.bin
RAR_FILE2=file2.rar

set -e
cd tests/fixtures

if [ ! -f "$FILE1" ]; then
    echo "$FILE1 not found! Will download.."
    wget -O $FILE1 http://www.opensubtitles.org/addons/avi/breakdance.avi
else
    echo "$FILE1 found! Skipping download.."
fi

if [ ! -f "$FILE2" ]; then
    echo "$FILE2 not found! Checking if rar file has already been downloaded.."
    
    if [ ! -f "$RAR_FILE2" ]; then
        echo "$RAR_FILE2 not found! Will download.."  
        wget -O $RAR_FILE2 http://www.opensubtitles.org/addons/avi/dummy.rar
        echo "NOTE: this file will be ~4 GB after extracting!"
    else
        echo "$RAR_FILE2 found, will proceed to extract.."
    fi

    7z e $RAR_FILE2
    mv dummy.bin $FILE2
else
    echo "$FILE2 found! Skipping download.."
fi

cd -
exit 0
