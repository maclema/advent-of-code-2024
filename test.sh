#!/bin/bash -e
cargo llvm-cov --html \
    --ignore-filename-regex '(_test\.rs|main.rs)' \
    --fail-under-functions 100 \
    --fail-under-lines 100 \
    --fail-under-regions 100
