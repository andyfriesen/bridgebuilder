#!/usr/bin/env zsh
set -ex

TESTLIB=./target/debug

cargo build
clang -I . main.cpp $TESTLIB/libbridgebuilder_demo.a -o main
