#!/bin/bash

cargo build --release
sudo cp target/release/lss /usr/local/bin/lss
