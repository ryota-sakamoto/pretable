pretable
===

You can create pretty table as follows.

```
+------------+-------+--------------+-------------+--------+
| REPOSITORY |  TAG  |   IMAGE ID   |   CREATED   |  SIZE  |
+------------+-------+--------------+-------------+--------+
|   ubuntu   | 18.04 | 452a96d81c30 | 6 weeks ago | 79.6MB |
|   ubuntu   | 18.04 | 452a96d81c30 | 7 weeks ago | 79.6MB |
+------------+-------+--------------+-------------+--------+
```

## benchmark

```
$ cargo +nightly bench --bench bench
    Finished bench [optimized] target(s) in 0.00s
     Running benches/bench.rs (target/release/deps/bench-2f3d06edc693d389)

running 4 tests
test bench_output_100    ... bench:      21,374 ns/iter (+/- 1,035)
test bench_output_1000   ... bench:     186,529 ns/iter (+/- 9,931)
test bench_output_10000  ... bench:   2,251,791 ns/iter (+/- 541,291)
test bench_output_100000 ... bench:  23,918,838 ns/iter (+/- 1,448,389)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 12.79s
```
