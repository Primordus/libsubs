
# libsubs

A small library for fetching subtitles from the internet.
Currently only supports English subtitles from OpenSubtitles.org.

Pull requests are welcome, as long as you write tests for your modifications :smile:

## Dependencies

All dependencies should be fetched by cargo except for the commands used by script/unzip_and_move.sh.
This bash script will be removed in the future and be replaced by pure Rust (when I have some more time).

## TODO

- multi-threaded downloads
- support other languages
- support other websites

