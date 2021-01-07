#!/bin/bash

sleep 1
cd "$WDIR"
diesel migration run
cargo watch -x 'run --release --target-dir /var/cache/cargo'
