# trigram

[![Build Status](https://travis-ci.org/ijt/trigram.svg?branch=master)](https://travis-ci.org/ijt/trigram)
[![License](https://img.shields.io/badge/license-Apache-blue.svg)](https://raw.githubusercontent.com/ijt/trigram/master/LICENSE)
[![Documentation](https://docs.rs/trigram/badge.svg)](https://docs.rs/trigram)

This Rust crate contains functions for fuzzy string matching.

It exports two functions. The `similarity` function returns the similarity of
two strings, and the `find_words_iter` function returns an iterator of matches
for a smaller string (`needle`) in a larger string (`haystack`).

The similarity of strings is computed based on their trigrams, meaning their
3-character substrings: https://en.wikipedia.org/wiki/Trigram.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
trigram = "0.2.2"
```

and call it like this:

```rust
use trigram::similarity;

fn main() {
	println!("{}", similarity(&"rustacean", &"crustacean"));
}
```

## Background
The `similarity` function in this crate is a reverse-engineered approximation
of the `similarity` function in the Postgresql pg\_trgm extension:
https://www.postgresql.org/docs/9.1/pgtrgm.html. It gives exactly the same
answers in many cases, but may disagree in others (none known). If you find a
case where the answers don't match, please file an issue about it!

A good introduction to the Postgres version of this is given on Stack Overflow:
https://stackoverflow.com/a/43161051/484529.
