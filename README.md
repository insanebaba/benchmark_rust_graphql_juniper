# Benchmarking for rust_juniper_actix

### Benchmark with load_runner.js(autocannon)
5 connections for 5 seconds

│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%  │ Avg     │ Stdev   │ Max       │
│ Latency │ 0 ms │ 0 ms │ 0 ms  │ 0 ms │ 0.01 ms │ 0.36 ms │ 111.62 ms │

┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 18431   │ 18431   │ 22863   │ 22927   │ 21982.4 │ 1779.49 │ 18421   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 3.91 MB │ 3.91 MB │ 4.85 MB │ 4.86 MB │ 4.66 MB │ 378 kB  │ 3.91 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

# Run server
1. cargo run --release

# Run benchmark
1. npm install -g autocannon
2. node ./load_runner.js
