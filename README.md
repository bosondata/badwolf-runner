# badwolf-runner

badwolf test runner docker image

This is a standard [Cargo](https://crates.io/) project,
 you will need to install [Rust](http://rust-lang.org/) to build it locally.


## Build image

Build python image for example:

```bash
$ docker build -t messense/badwolf-test-runner:python -f dockerfiles/python.Dockerfile .
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
