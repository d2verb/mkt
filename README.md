# mkt - MaKe a Temporary note
[![CI](https://github.com/d2verb/mkt/workflows/test/badge.svg)](https://github.com/d2verb/mkt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/d2verb/mkt/blob/master/LICENSE)

A tool to make a temporary note

## Usage
### help
```bash
$ mkt --help
mkt 0.1.0
A tool to make a temporary note

USAGE:
    mkt [FLAGS] [OPTIONS] [prefix]

FLAGS:
    -h, --help       Prints help information
    -o, --open       Opens editor to edit note
    -V, --version    Prints version information

OPTIONS:
    -e, --editor <editor>          Editor to edit temporary note
    -x, --extension <extension>    Extension of temporary note

ARGS:
    <prefix>    Prefix of temporary note
```

Note that the default value of `editor` is `vim` and the default value of `extension` is `md`.

### Example
#### 1. make note
```bash
$ mkt
$ ls
2020_11_14_19_13_45_y7vAA0a7.md
$ mkt mtg # you can also pass the prefix of the generated file
$ ls
2020_11_14_19_13_45_y7vAA0a7.md  mtg_2020_11_14_19_16_16_y0Pdor7A.md
```

#### 2. make and edit note
```bash
$ mkt -o
[launch editor...]
```

## Configuration
If `$HOME/.mktrc` exists, mkt use it as a config file. The config file must consist of multiple lines like `key = value`.
```bash
$ cat ~/.mktrc
extension = "txt"
prefix = "mtg"
```

## LICENSE
MIT
