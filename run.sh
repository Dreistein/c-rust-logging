#!/bin/bash


SCRIPT_DIR=$(dirname "$0")
pushd $SCRIPT_DIR
# start

set -e

pushd log_lib
cargo build --release
mkdir -p ../c_proj/lib
cp target/release/liblog_lib.a ../c_proj/lib/liblog_lib.a
popd

pushd c_proj
cmake -S . -B /tmp/log_test
make -j$(nproc) -C /tmp/log_test
popd

pushd /tmp/log_test
echo "Running application..."
echo "--------------------------------"
./log_test
echo "--------------------------------"
echo "Application finished"
popd

# end
popd