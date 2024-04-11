#!/usr/bin/sh
cargo run -- -i $1 -m $2 -d $3.dot
dot -Tpdf $3.dot -o $3
rm $3.dot
