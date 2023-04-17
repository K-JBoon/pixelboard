#!/bin/bash
ARCH="aarch64-unknown-linux-gnu"

cross build --release --target=$ARCH
scp target/$ARCH/release/pixelboard root@dietpi:/app/pixelboard/pixelboard
