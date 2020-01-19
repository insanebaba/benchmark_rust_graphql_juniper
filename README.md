# Benchmarking for rust_juniper_actix

### Benchmark with load_runner.js(autocannon)
5 connections for 5 seconds

| Stat    | 2.5% | 50% | 97.5% | 99% | Avg    | Stdev  | Max      |
|---------|-----|-----|-------|-----|--------|--------|----------|
| Latency | 0ms | 0ms | 0ms   | 0ms | 0.01ms | 0.36ms | 111.62ms |

| Stat      | 1%     | 2.5%   | 50%    | 97.5%  | Avg     | Stdev   | Min    |
|-----------|--------|--------|--------|--------|---------|---------|--------|
| Req/Sec   | 18431  | 18431  | 22863  | 22927  | 21982.4 | 1779.49 | 18421  |
| Bytes/Sec | 3.91MB | 3.91MB | 4.85MB | 4.86MB | 4.66MB  | 378KB   | 3.91MB |

# Run server
1. cargo run --release

# Run benchmark
1. npm install -g autocannon
2. node ./load_runner.js
