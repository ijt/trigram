# trigram

This Rust crate contains a function to compute trigram-based similarity of strings.

A trigram is a 3-character long string, and it turns out that the set of all
the trigrams in a given string is a good way to represent it when computing its
similarity to other strings.

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
