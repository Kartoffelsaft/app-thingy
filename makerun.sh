#!/bin/sh

tsc -p ./client/ &&
browserify ./client/build/* > ./client/app.js
cargo run
