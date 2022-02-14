#!/bin/bash
set -ex

cd rp-hal
cargo build --release --example blinky


echo 'Plug on your Raspberry Pico with pressing boot button'

#pause
read -s -n 1 -p "Press any key to continue . . ."


cargo run --release --example blinky

