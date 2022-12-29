# nonparallelex

[![CircleCI][circle-ci-badge]][circle-ci]
[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
[![License][license-badge]][license]

A fork of [`nonparallel`](https://github.com/dbrgn/nonparallel) with
user-specified locking expressions.

A procedural macro for Rust that allows you to ensure that functions (e.g. unit
tests) are not running at the same time.
This is especially useful for integration tests,
where tests that are writing to the same database table should not run in
parallel.

This is achieved by executing the provided expression (a mutex lock,
for instance) the beginning of the annotated function and holding on to
the returned value (for instance, the lock guard), such that the lock
is released when the function returns (or panics).

This is useful to inject locks into functions that altered by other macros,
like `tokio::test`, and where you can’t inject your lock expression by other
means.

Macros are applied from top to bottom, when using with `tokio:test`
and similar macros, be sure to insert the nonparallel call last.
This will wrap the entire runtime with the lock.
If you insert nonparallel above the `tokio::test`, the lock will be executed
inside the runtime, which means the runtime will shutdown after the lock
is released, and may lead to things you meant to be under mutex to
run outside of mutex until the runtime actually shuts down.

## Usage

```rust
use tokio::sync::Mutex;
use nonparallelex::nonparallel;

// Create two locks
static MUT_A: Mutex<()> = Mutex::const_new(());
static MUT_B: Mutex<()> = Mutex::const_new(());

// Mutually exclude parallel runs of functions using those two locks.

#[tokio::test]
#[nonparallel(MUT_A.blocking_lock())]
async fn function_a1() {
    // This will not run in parallel to function_a2.
}

#[tokio::test]
#[nonparallel(MUT_A.blocking_lock())]
async fn function_a2() {
    // This will not run in parallel to function_a1.
}

#[tokio::test]
#[nonparallel(MUT_B.blocking_lock())]
async fn function_b() {
    // This may run in parallel to function_a*.
}
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

<!-- Badges -->
[circle-ci]: https://circleci.com/gh/dbrgn/nonparallel/tree/master
[circle-ci-badge]: https://circleci.com/gh/dbrgn/nonparallel/tree/master.svg?style=shield
[crates-io]: https://crates.io/crates/nonparallel
[crates-io-badge]: https://img.shields.io/crates/v/nonparallel.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/nonparallel
[crates-io-download-badge]: https://img.shields.io/crates/d/nonparallel.svg?maxAge=3600
[license]: https://github.com/dbrgn/nonparallel#license
[license-badge]: https://img.shields.io/badge/License-Apache%202.0%20%2f%20MIT-blue.svg
