# RBGN

RBGN - A complete rewrite of [Bashgenn](https://github.com/romw314/bashgenn) with the support for interpreting, written in Rust.

[![Crates.io Version](https://img.shields.io/crates/v/rbgn)](https://crates.io/crates/rbgn)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/rbgn)](https://crates.io/crates/rbgn)
[![GitHub License](https://img.shields.io/github/license/romw314/rust-bashgenn)](https://github.com/romw314/rust-bashgenn/blob/master/UNLICENSE.txt)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/romw314/rust-bashgenn/rust.yml?logo=githubactions&logoColor=white)](https://github.com/romw314/rust-bashgenn/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rbgn?logo=docsdotrs)](https://docs.rs/rbgn)
[![GitHub contributors](https://img.shields.io/github/contributors/romw314/rust-bashgenn?logo=github)](https://github.com/romw314/rust-bashgenn/graphs/contributors)
[![GitHub forks](https://img.shields.io/github/forks/romw314/rust-bashgenn?style=flat&logo=github)](https://github.com/romw314/rust-bashgenn/forks)
[![GitHub issues](https://img.shields.io/github/issues/romw314/rust-bashgenn?logo=github)](https://github.com/romw314/rust-bashgenn/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/romw314/rust-bashgenn?logo=github)](https://github.com/romw314/rust-bashgenn/pulls)
[![GitHub Repo stars](https://img.shields.io/github/stars/romw314/rust-bashgenn?style=flat&logo=github)](https://github.com/romw314/rust-bashgenn/stargazers)

## Installation

First you need to [install Rust](https://rustup.rs/), if you don't already.

Then use this command to install RBGN:

```sh
cargo install rbgn
```

See [the wiki](https://github.com/romw314/rust-bashgenn/wiki/Installation) for more installation options.

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
