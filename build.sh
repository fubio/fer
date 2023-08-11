#!/bin/sh
LEASE_CSV="lease.csv"
RI_CSV="ri.csv"
TD_CSV="td.csv"
export RUST_BACKTRACE=1
export RUSTFLAGS="-Awarnings"
for dir in ./polybench/*/
do
  dir=${dir%*/}
  dir=${dir##*/}
  echo "Processing $dir"
  TD_CSV=./polybench/$dir/td_$dir.csv
#  echo "TD CSV: $TD_CSV"
    for file in ./polybench/$dir/*
    do
      if [[ $file == *"lease"* ]]
      then
        LEASE_CSV=$file
      fi

      if [[ $file == *.txt ]]
      then
        RI_CSV=$file
      fi
    done
#    echo "Lease CSV: $LEASE_CSV"
#    echo "RI CSV: $RI_CSV"
    cargo run --bin td_generator -- --lease-csv $LEASE_CSV --ri-csv $RI_CSV --td-csv $TD_CSV
    cargo run --release --bin fer -- --td-csv $TD_CSV

#    cargo run --release --bin fer -- --td-csv $TD_CSV --dir $dir
done

