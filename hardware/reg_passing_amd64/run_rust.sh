rustc +nightly  -C  target_cpu=native -C opt_level=3 -Zdefault_hidden_visibility=yes  passing.rs
echo "by value"
time ./passing 0 100000000 
echo "by many parameters"
time ./passing 1 100000000
echo "by ref"
time ./passing 2 100000000