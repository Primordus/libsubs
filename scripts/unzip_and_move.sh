#!/bin/bash

# TODO try to remove this later and replace with pure Rust for portability!

set -e

ZIP_LOCATION=$1
UNZIP_DIR=$2
EPISODE_NAME=$3

# Change movie extension to .srt
SRT_FILE_NAME="${EPISODE_NAME%.*}.srt"

# Unzip to a certain location
unzip -o $ZIP_LOCATION -d $UNZIP_DIR
cd $UNZIP_DIR

# Find the .srt file
GREP_OUTPUT=$(ls | grep ".srt" | head -1)
WC_OUTPUT=$(echo "$GREP_OUTPUT" | wc -l)

if [ "$WC_OUTPUT" -ne "1" ]; then
    echo "Found no subtitles for $EPISODE_NAME!"
    exit 1
fi

# Move .srt file to the same directory as the episode
mv $GREP_OUTPUT $SRT_FILE_NAME

cd -
exit 0

