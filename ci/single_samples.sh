#!/usr/bin/env bash

#XARGO_SGX = 0 or 1 is provided by drone
#SGX_MODE = HW or SW is provided by drone

RUST_SDK_ROOT=${PWD}
RUST_TARGET_PATH=${RUST_SDK_ROOT}"/xargo/"

SINGLE_EXAMPLES=(backtrace \
                 crypto \
                 file \
                 hello-regex \
                 hello-rust \
                 hello-rust-vscode-debug \
                 helloworld \
                 kvdb-memdb \
                 localattestation \
                 machine-learning \
                 net2 \
                 protobuf \
                 sealeddata \
                 serialize \
                 switchless \
                 thread \
                 unit-test \
                 wasmi \
                 zlib-lazy-static-sample)

#$0 is the script itself
#$1 is the first argument -- sample name
test_one_single() {
    cd ${RUST_SDK_ROOT}/samplecode/$1 && \
    make && \
    cd bin && \
    ./app && echo "Success!"
}

for i in ${SINGLE_EXAMPLES[@]}
do
    test_one_single ${i}
done
