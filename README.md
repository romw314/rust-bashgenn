# RBGN

RBGN - A complete rewrite of [Bashgenn](https://github.com/romw314/bashgenn) with the support for interpreting. Written in Rust.

## Installation

First you need to [install Rust](https://rustup.rs/), if you don't already.

Then use this command to install RBGN:

```sh
cargo install --git https://github.com/romw314/rust-bashgenn.git
```

## CLI Usage

```
Usage: rbgn [OPTIONS] <FILE>

Arguments:
  <FILE>  File to build

Options:
  -o, --output <FILE>  Specify the output file
  -i, --interpret      Interpret the script instead of compiling it to Bash
  -h, --help           Print help
  -V, --version        Print version
```

## Usage

```sh
$ cat reverse.bgn
STATIC_STR_VAR << <<
STATIC_STR_VAR >> >>
STATIC_STR_SPACE space

NONL <<
NONL space
READ x

NONL >>
NONL space

STRGET x
	STORELAST x y
	NONL y
DONE

ECHO
$ rbgn -i reverse.bgn
<< Hey, this is perfect!
>> !tcefrep si siht ,yeH
```
