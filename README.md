# UNorm rounding benchmark

This repo contains the benchmark for multiple implementations of unorm conversions in Rust.

## Results

This is the output of running `cargo bench` on my machine. The relevant specs(?) of my machine:

- OS: Windows 10
- CPU: Intel® Core™ i7-8700K CPU @ 3.70GHz
- Rust: 1.78.0

```
PS C:\Users\micha\Git\rounding-rs> cargo bench
   Compiling rounding-rs v0.1.0 (C:\Users\micha\Git\rounding-rs)
    Finished `bench` profile [optimized] target(s) in 6.60s
     Running unittests src\lib.rs (target\release\deps\rounding_rs-d4cdbb4d41c4eab4.exe)

running 1 test
test tests::test_correctness ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\bench.rs (target\release\deps\bench-853108463e15b11c.exe)
Gnuplot not found, using plotters backend
u5_to_u8_naive          time:   [100.60 µs 100.95 µs 101.35 µs]
                        change: [-4.4668% -2.9545% -1.6724%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  10 (10.00%) high mild
  3 (3.00%) high severe

u5_to_u8_v2             time:   [17.741 µs 17.842 µs 17.955 µs]
                        change: [-0.9296% +0.0753% +0.9711%] (p = 0.87 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

u5_to_u8_unsafe         time:   [7.3612 µs 7.4211 µs 7.4950 µs]
                        change: [+1.4705% +2.9445% +4.3346%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

u5_to_u8_safer          time:   [7.7434 µs 7.7760 µs 7.8144 µs]
                        change: [-0.9831% -0.1113% +0.7896%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  8 (8.00%) high mild
  6 (6.00%) high severe

u5_to_u8_safer_int      time:   [7.3363 µs 7.4120 µs 7.5003 µs]
                        change: [-5.8513% -4.8887% -3.8761%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

u5_to_u8_int            time:   [5.4164 µs 5.4840 µs 5.5627 µs]
                        change: [+7.5426% +9.7796% +12.352%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)
  8 (8.00%) high mild
  8 (8.00%) high severe

u5_to_u8_bcdec          time:   [4.6062 µs 4.6210 µs 4.6378 µs]
                        change: [+0.7544% +2.4523% +4.7191%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
```

## License

All code in this repo is licensed under The Unlicense OR CC0-1.0. Whatever gives you more freedom.
