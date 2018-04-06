#!/bin/bash
RUST_TARGET_PATH=$(pwd) xargo build --release && ./gdb.sh
