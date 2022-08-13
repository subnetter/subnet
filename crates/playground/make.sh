#!/bin/bash

DIR="$(dirname "$0")"

if cargo build "$@"; then
    [ -d "$DIR/target/debug" ] && cp -r "$DIR/resources" "$DIR/target/debug"
    [ -d "$DIR/target/release" ] && cp -r "$DIR/resources" "$DIR/target/release"
fi
