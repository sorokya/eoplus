# EOPlus Parser/Lexer for Rust

[![Build Status][actions-badge]][actions-url]
[![Crate][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/eoplus.svg
[crates-url]: https://crates.io/crates/eoplus
[docs-badge]: https://img.shields.io/docsrs/eoplus.svg
[docs-url]: https://docs.rs/eoplus
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/sorokya/eoplus/blob/master/LICENSE
[actions-badge]: https://github.com/sorokya/eoplus/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/sorokya/eoplus/actions/workflows/rust.yml

This crate can parse EOPlus input into Quest data structures. It is built from the [eoplus-antlr4](https://github.com/Cirras/eoplus-antlr4) grammar written by @cirras.

Currently in use by [reoserv](https://reoserv.net) for quest parsing.

# Issues

Currently quests with invalid syntax will parse without error. The [antlr4-rust](https://github.com/rrevenantt/antlr4rust) crate dumps some errors to the console but this crate assumes that everything is fine.
I plan to fix this in a future release.
