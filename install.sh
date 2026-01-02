#!/bin/sh

cargo build --release
cp target/release/mother /usr/local/bin/mother

