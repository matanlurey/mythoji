# mythoji

A minimal Rust crate that helps identify and display fantasy appropriate emojis.

[![Rust Checks](https://github.com/matanlurey/mythoji/actions/workflows/rust.yml/badge.svg)](https://github.com/matanlurey/mythoji/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/matanlurey/mythoji/badge.svg)](https://coveralls.io/github/matanlurey/mythoji)
[![Current Crates.io Version](https://img.shields.io/crates/v/mythoji.svg)](https://crates.io/crates/mythoji)
[![Docs](https://docs.rs/mythoji/badge.svg)](https://docs.rs/mythoji/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Why `mythoji`?

- You are building a fantasy game that wants to use minimal graphics.
- You are tired of Googling for useful emoji.
- You don't want to use a more [general purpose crate][emoji].

[emoji]: https://crates.io/crates/emoji

```rust
use mythoji::{Emoji, Gender, Person, Location, SkinTone};

let castle = Location::Castle;
assert_eq!(castle.to_string(), "🏰");

let female_elf = Emoji::Person(Person::Elf, SkinTone::Neutral, Gender::Female);
assert_eq!(female_elf.to_string(), "🧝‍♀️");
```
