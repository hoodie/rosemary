<div align="center">

<h1>
<small>Parsley, Sage,</small>
<strong>Rosemary</strong>,
<small> but no Thyme </small>
</h1>

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/rosemary/ci.yml?branch=main)](https://github.com/hoodie/rosemary/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/rosemary)](https://crates.io/crates/rosemary)
[![contributors](https://img.shields.io/github/contributors/hoodie/rosemary)](https://github.com/hoodie/rosemary/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2023)

[![version](https://img.shields.io/crates/v/rosemary)](https://crates.io/crates/rosemary/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/rosemary/)
[![license](https://img.shields.io/crates/l/rosemary.svg?style=flat)](https://crates.io/crates/rosemary/)

A tiny command line tool that runs your command for you and tries to tell you how much longer the damn thing is going to take.

</div>

Do you also have to run long running scripts like tests or build jobs which do not tell you how long they are going to take?

Rosemary is a tool similar to the shell built-in `time`,
but it does not only tell you how long a job took,
it tells you how long it is already running and if it knows how long the job took previously it will even render a progressbar.

So you know if it is worth getting a coffee or not.

![rosemary demo](./rosemary.gif)

## Status

Rosemary is not particularly smart yet,
it remembers the previous run of a certain command in a certain folder,
better classifications are not implemented yet.

## License

rosemary is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/rosemary/issues) or comprehensive pull requests are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in rosemary by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
