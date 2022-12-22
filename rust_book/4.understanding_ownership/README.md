# Ownership

It enables Rust to make memory safe guarantees without needing a garbage collector.

## Ownership rules

- Each value in Rust has an owner;
- There can only be one owner at a time;
- When the owner goes out of scope, the value will be dropped;
