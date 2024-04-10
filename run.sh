#!/usr/bin/sh
cargo run > res.dot
dot -Tpdf res.dot -o res.pdf
