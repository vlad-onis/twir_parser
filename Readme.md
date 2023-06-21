# TWIR parser

The goal of this CLI project is to parse the twir (This Week In Rust) articles in search of given keywords.

## Introduction
This CLI tool was born because TWIR newsletter is an amazing learning resource. This tool allows you to search for given word patterns in the
newsletter's archive. It has 2 types of search online and offline. In the beginning if you've never used it before, it will fetch all the content
from the newsletter, this may take a few minutes. After fetch it will decide to search online or offline based on a CLI option that you can pass
or automatically in case the cache file exists on your system. The tool defaults to offline search for speed.

## Usage

```bash
# Searches the entire TWIR issue archive for "embedded audio" articles
cargo run -- --search "embedded audio"

# Online search with limiting the search to the most recent 10 issues
cargo run --release -- --search "ESP32" --online --limit 10
```
