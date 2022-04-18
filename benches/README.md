# Benchmarks

This folder contains benchmark tests that can be used to
(roughly) measure performance of the `serde-this-or-that` crate.

## Quickstart

Start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/serde-this-or-that.git
```

Then, simply `cd` into the project folder:

```shell
❯❯ cd serde-this-or-that
```

From here, you can run the benchmark tests with:

```shell
❯❯ cargo bench
```

To run only a specific benchmark, you can pass the `--bench` argument.

For instance, here's an example of only benchmarking the `serde_with` tests:

```shell
❯❯ cargo bench --bench serde_with
```
