#!/bin/bash
RUST_TARGET_PATH=$(pwd) xargo build --release \
    && echo -e "c\nq\n" | arm-none-eabi-gdb -iex 'add-auto-load-safe-path .' -ex "tar ext :4242" -ex "load-reset" target/stm32f7/release/mp-pong-stm32f7


