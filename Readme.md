# TWIR(This week in rust) parser

The goal of this CLI project is to parse the TWIR articles in search of given keywords.

## Introduction
This CLI tool was born because TWIR newsletter is an amazing learning resource. This tool allows you to search for given word patterns in the
newsletter's archive. It has 2 types of search online and offline. In the beginning if you've never used it before, it will fetch all the content
from the newsletter, this may take a few minutes. After fetch it will decide to search online or offline based on a CLI option that you can pass
or automatically in case the cache file exists on your system. The tool defaults to offline search for speed.

## Installation

In order to install, first clone this repository:
```bash
git clone git@github.com:vlad-onis/twir_parser.git
```

And then you can install it on your local machine by going into the directory you just cloned and run:
In order to install, first clone this repository:
```bash
cargo install --path .
```

## Usage

```bash
# Searches the entire TWIR issue archive for "embedded audio" articles
# This will also save a local file and will use that as cache on next runs
twir_parser --search "embedded audio"

# Online search with limiting the search to the most recent 10 issues
twir_parser --search "ESP32" --online --limit 10
```
