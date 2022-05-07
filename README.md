# ID
An I/D Machine Implementation in Rust. Learn about I/D Machine
[here](https://esolangs.org/wiki/I/D_machine).

A precompiled binary is provided in the releases tab of this
GitHub repository for Linux. For all other operating systems,
local compilation is necessary.

Building from source is supported on all platforms supported by
the Rust language toolchain. To build, first clone the repository
(`git clone https://github.com/nekodjin/id.git`), or download
the compressed source code in the releases tab. Then, invoke
the command `cargo build --release`. The resultant executable will
be located in `target/release/`. The program can be compiled and
executed in one step with `cargo run --release`.

The program reads utf8-encoded text from the standard input stream.
All `I` characters correspond to the `I` instruction, all `D`
characters correspond to the `D` instruction, and all other
characters are ignored. These instructions will then be executed,
in order, in a loop, until either the program is aborted or the
program consumes so much RAM that your operating system terminates
it. Results of computation are not observable at this time. In
other words, the interpreter can be summarized as such: "you
press run and it gets hot."
