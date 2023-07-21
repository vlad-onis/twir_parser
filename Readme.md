<p align="center">
    <img src="logo.png" alt="logo" width=350/>
</p>

<h2 align="center">TWIR(This week in rust) crawler</h2>
The goal of this CLI project is to parse the TWIR articles in search of given keywords.

## Introduction
This CLI tool was born because TWIR newsletter is an amazing learning resource. This tool allows you to search for given word patterns in the
newsletter's archive. It has 2 types of search online and offline. In the beginning if you've never used it before, it will fetch all the content
from the newsletter, this may take a few minutes. After fetch it will decide to search online or offline based on a CLI option that you can pass
or automatically in case the cache file exists on your system. The tool defaults to offline search for speed.

## Installation

In order to install, run

```bash
cargo install --git https://github.com/vlad-onis/twir_parser
```

## Usage

```bash
# Searches the entire TWIR issue archive for "embedded audio" articles
# This will also save a local file and will use that as cache on next runs
twir_parser --search "embedded audio"

# Online search with limiting the search to the most recent 10 issues
twir_parser --search "ESP32" --online --limit 10
```
