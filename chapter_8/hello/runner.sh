#!/bin/sh
CARGO_MANIFEST_DIR=$(pwd)
docker run --rm -v $CARGO_MANIFEST_DIR:$CARGO_MANIFEST_DIR \
qemu \
sh -c "cd $CARGO_MANIFEST_DIR; \
qemu-system-gnuarmeclipse \
    -cpu cortex-m4 \
    -machine STM32F4-Discovery \
    -nographic \
    -semihosting \
    -kernel $1"