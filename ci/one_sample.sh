#!/usr/bin/env bash

#XARGO_SGX = 0 or 1 is provided by drone
#SGX_MODE = HW or SW is provided by drone

RUST_SDK_ROOT=${PWD}
RUST_TARGET_PATH=${RUST_SDK_ROOT}"/xargo/"

#$0 is the script itself
#$1 is the first argument -- sample name
cd samplecode/$1 && \
make && \
cd bin && \
./app
