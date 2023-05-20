pretable
===

[![test](https://github.com/ryota-sakamoto/pretable/actions/workflows/test.yaml/badge.svg?branch=master)](https://github.com/ryota-sakamoto/pretable/actions/workflows/test.yaml)

You can create pretty table as follows.

```
+--------------+----------------+--------+
| NAME         |     FORMAL     | HEIGHT |
+--------------+----------------+--------+
| Everest      |  Chomolungma   |  8848m |
| K2           | Karakorum No.2 |  8611m |
| Kanchenjunga |                |  8505m |
+--------------+----------------+--------+
```

## benchmark

```
$ cargo +nightly bench --bench bench
    Finished bench [optimized] target(s) in 0.00s
     Running benches/bench.rs (target/release/deps/bench-2f3d06edc693d389)

running 4 tests
test bench_output_100    ... bench:      20,188 ns/iter (+/- 1,280)
test bench_output_1000   ... bench:     185,939 ns/iter (+/- 15,162)
test bench_output_10000  ... bench:   2,276,584 ns/iter (+/- 697,180)
test bench_output_100000 ... bench:  23,073,557 ns/iter (+/- 2,902,075)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 14.92s
```
