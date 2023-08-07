#!/bin/sh
LEASE_CSV="lease.csv"
RI_CSV="ri.csv"
TD_CSV="td.csv"
export RUST_BACKTRACE=1
#cargo run --bin td_generator -- --lease-csv $LEASE_CSV --ri-csv $RI_CSV
cargo run --bin fer -- --td-csv $TD_CSV
