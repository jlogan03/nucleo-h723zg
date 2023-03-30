[![build](https://github.com/jlogan03/nucleo-h723zg/actions/workflows/cargo.yml/badge.svg)](https://github.com/astraw/nucleo-h743zi/actions/workflows/cargo.yml)

# nucleo-h723zg

Example programs for the NUCLEO-H23ZG dev board.


## Pre-requisites

You will need the following components installed before building the project.

```
$ rustup target add thumbv7em-none-eabihf
$ rustup component add llvm-tools-preview
$ cargo install cargo-binutils cargo-flash
```
This will install a new target for the Rust compiler supporting Cortex-M7F and a
cargo plugin to call `binutils` directly.

For debugging the program, you can use either probe-rs or a compatible version
of `gdb` for your system.

## Building and running

The quickest way to a working flash workflow is to use cargo-flash via the included script:

```bash
sh flash.sh blinky
```

replacing "blinky" with another example file as needed.

For some alternative approaches to flashing and debugging, 
see https://github.com/astraw/nucleo-h743zi .

## License

[0-clause BSD license](LICENSE-0BSD.txt).
