<p align="center">
     <img src="logo.png" alt="logo" width=350/>
 </p>

 <h2 align="center">TWIR(This week in rust) crawler</h2>

The goal of this CLI project is to parse the TWIR articles in search of given keywords.

## Introduction
This CLI tool was born because TWIR newsletter is an amazing learning resource. This tool allows you to search for given word patterns in the
newsletter's archive. 

It has 2 types of search online and offline. In the beginning if you've never used it before, fetch all the content
from the newsletter - please not that this may take a bit (20-30 seconds). After the fetch it will decide to search online or offline based on a CLI option that you can pass
or automatically in case the cache file exists on your system. The tool defaults to offline search for speed.

## Installation

In order to install, run

```bash
cargo install --git https://github.com/vlad-onis/twir_parser
```

## Usage

```bash

# Prefered usage for now is to clone this repo and run for example:
cargo run --release -- --search "async trait" --limit 30

# Searches the entire TWIR issue archive for "embedded audio" articles
# This will also save a local file and will use that as cache on next runs
twir_parser --search "embedded audio"

# Online search with limiting the search to the most recent 10 issues
twir_parser --search "ESP32" --online --limit 10
```
