# free-cpus
Get the set of free CPU cores on your Linux machine.

- [docs.rs](https://docs.rs/free-cpus)
- [crates.io](https://crates.io/crates/free-cpus)

The logic is heavily inspired by [AFL++'s code](https://github.com/AFLplusplus/AFLplusplus/blob/85c5b5218c6a7b2289f309fbd1625a5d0a602a00/src/afl-fuzz-init.c#L109-L452).

## Usage

Add to Cargo.toml:

```toml
[dependencies]
free-cpus = "2.0.0"
```

In your rust code:

```rust
// Get the set of free CPU cores on this Linux machine
let cpus: HashMap<usize> = free_cpus::get().unwrap();
```
