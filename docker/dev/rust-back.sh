#!/bin/bash

sleep 1
cd "${WDIR}"
diesel migration run
cargo watch --workdir "${WDIR}" -x 'run --target-dir /var/cache/cargo --release'
