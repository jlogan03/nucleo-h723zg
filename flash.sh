#!/bin/bash

# First, do a test build with a fake startup to check for panic branches
# that occur after startup
cargo build --release --bin $1 --features=test

# Do a new build that includes a real startup & flash
cargo flash --release --chip=STM32H723ZGTx --bin $1
