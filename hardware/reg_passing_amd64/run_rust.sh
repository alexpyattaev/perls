rustc -C  target_cpu=native -C opt_level=3 passing.rs
time ./passing 0 100000000
time ./passing 1 100000000
