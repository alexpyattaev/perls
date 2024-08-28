echo "let us use a different way to call the same function because we can..."
clang -O3 impl.c -c -o impl.o
clang -O3 harness.c impl.o -o benchmark
time ./benchmark 0 1000000000
time ./benchmark 1 1000000000
