#!/bin/bash
#RUST_TARGET_PATH=$(pwd) xargo build 
echo -e "c\nq\n" | cargo run --release
