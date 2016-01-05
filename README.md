# badwolf-runner

badwolf test runner docker image

This is a standard [Cargo](https://crates.io/) project,
 you will need to install [Rust](http://rust-lang.org/) to build it locally.


## Build image

```bash
$ docker build -t badwolf-test-runner
```

## Build runner locally

```bash
$ cargo build --release
```

## Run runner locally

```bash
$ cargo run
```

## Run tests

```bash
$ cargo test
```
