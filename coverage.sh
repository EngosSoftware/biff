#!/usr/bin/env bash

###############################################################################
# Dependencies:
#
# $ sudo dnf install lcov
# $ rustup component add llvm-tools-preview
# $ cargo install grcov
#
###############################################################################

WORKING_DIR=$(pwd)
CARGO_NAME=$(grep -oE '^name = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+')
CARGO_VERSION=$(grep -oE '^version = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+')
BINARY_PATH="$WORKING_DIR"/target/debug
BBT_DIR="$WORKING_DIR"/bbt

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
# run all tests including including black-box tests
cargo +nightly test
# build the whole binary before running tests
cargo +nightly build
# run black-box tests to collect the coverage of the code executed from command-line
echo "$BBT_DIR"
if [[ -d "$BBT_DIR" ]]
then
  export PATH=$BINARY_PATH:$PATH
  cd "$BBT_DIR" || exit 1
  ./run.sh
  cd "$WORKING_DIR" || exit 1
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage
# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*tests*" -o ./target/lcov/lcov.info
# generate coverage report
genhtml -t "$CARGO_NAME v$CARGO_VERSION" -q -o ./target/coverage ./target/lcov/lcov.info
# display final message
echo ""
echo "Open coverage report: file://$WORKING_DIR/target/coverage/index.html"
echo ""

