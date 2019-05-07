# cb

[![Build Status](https://travis-ci.org/yaa110/cb.svg?branch=master)](https://travis-ci.org/yaa110/cb)

Command line interface to manage clipboard

## How to install

- run `make && sudo make install`

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