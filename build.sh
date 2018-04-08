#!/bin/bash
RUST_TARGET_PATH=$(pwd) xargo build \
    && arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -ex "tar ext :4242" -ex "load-reset" target/stm32f7/debug/mp-pong-stm32f7


