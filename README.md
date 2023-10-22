# lice

[![Build Status]][actions]
[![License]][mit-license]
[![Docs]][Docs-rs]
[![Latest Version]][crates.io]
[![rustc 1.31+]][Rust 1.31]

[Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/lice/ci.yml?branch=main
[actions]: https://github.com/refcell/lice/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/lice.svg
[crates.io]: https://crates.io/crates/lice
[rustc 1.31+]: https://img.shields.io/badge/rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/license-MIT-7795AF.svg
[mit-license]: https://github.com/refcell/lice/blob/main/LICENSE.md
[Docs-rs]: https://docs.rs/lice/
[Docs]: https://img.shields.io/docsrs/lice.svg?color=319e8c&label=docs.rs


**Dead simple, minimal SPDX License generator library written in Rust.** Lice is in https://github.com/refcell/lice/labels/beta.

![](./etc/banner.png)

**[Install](#usage)**
| [User Docs](#what-is-lice)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)
| [Attribution](#attribution)

## What is lice?

`lice` is a dead simple, minimal library for generating
valid SPDX licenses. It was primarily built for `amble`
but ripped out into it's own crate to provide an extensible
library, published on [crates.io][crates.io].

## Usage

Install `lice` with cargo.

```ignore,sh,no_run
cargo add lice
```

Alternatively, build it from source.

```ignore,sh,no_run
git clone git@github.com:refcell/lice.git && cd lice
cargo build --release
```

## Contributing

All contributions are welcome! Experimentation is highly encouraged
and new issues are welcome.

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/lice/issues/new)
if no relevant issue already exists.

## Attribution

Much of this work is based off of [lic][lic-repo], an spdx license
generator binary that isn't extensible as a library. The adapted
[`lice`][crates.io] crate extends the SPDX "API" to
provide more verbose license fetching methods. Big h/t to the
creators of [lic][lic-repo] especially [SigureMo][sig-mo]

[sig-mo]: https://github.com/SigureMo
[lic-repo]: https://github.com/ShigureLab/lic/tree/main

## License

This project is licensed under the [MIT License](LICENSE.md).
Free and open-source, forever.
*All our rust are belong to you.*
