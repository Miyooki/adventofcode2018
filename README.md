# Advent Of Code 2018

## Made with Rust 1.3x
I wanted to try to learn Rust and thought it'd be fun to apply it to the challenges from AOC.

## Benchmark Tests 
#### Using a i7-6700HQ CPU, 16GB of 2133MHz RAM, and a PM951 512GB for storage
### Optimizations:
* Setting lto=true and codegen-units=1 in release build (saves a few ms per day).
#### Day 1:
```
Part 1: The resulting frequency is 516
Benchmark: 34.371µs
-----------------------------------------------
Part 2: The first repeating frequency is 71892
Benchmark: 28.589ms
```
#### Day 2:
```
Part 1: The candidate checksum is 8398
Benchmark: 167.902µs
-----------------------------------------------
Part 2: The correct box ID is hhvsdkatysmiqjxunezgwcdpr
Benchmark: 1.073ms
```
### Day 3:
```
Part 1: The amount of overlap is 118858
Benchmark: 2.982ms
-----------------------------------------------
Part 2: The non-overlapping claim ID is 1100
Benchmark: 11.653ms
```
### Day 4:
```

```