# cb

[![Build Status](https://travis-ci.org/yaa110/cb.svg?branch=master)](https://travis-ci.org/yaa110/cb) [![Download](https://img.shields.io/badge/download-0.1.0-blue.svg)](https://github.com/yaa110/cb/releases/download/0.1.0/cb)

Command line interface to manage clipboard

## How to install

### Pre-Compiled

you can download a [pre-compiled executable](https://github.com/yaa110/cb/releases), then you should copy that executable to `/usr/bin` or add it to your `$PATH` env. Do not forget to `chmod +x cb`.

### Build Manually

- Install rust: `curl -sSf https://sh.rustup.rs | sh`
- Install packages: `xorg-dev` and `build-essential`
- Run `make && sudo make install`

## How to use

- Copy text: `cb -t "Text to be copied"`
- Paste copied text: `cb -p`
- Copy from stdin: `cat file | cb`

## Usage

```sh
Usage: cb [OPTIONS]

Optional arguments:
  -h, --help       Prints the help message
  -V, --version    Prints the version
  -p, --paste      Pastes the content of clipboard
  -c, --clear      Clears the content of clipboard
  -s, --server     Starts server as a daemon
  -r, --raw        Do not print newline after pasting the content
  -t, --text TEXT  Store TEXT into clipboard
```