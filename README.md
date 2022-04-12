# Read BAM header on an AWS lambda with Noodles

This is a small Bioinformatics proof of concept that bundles [noodles](http://github.com/zaeleus/noodles) into an AWS Lambda.

A previous lambda was written using the C-bindgen-based [rust-htslib](https://github.com/brainstorm/s3-rust-htslib-bam). This iteration gets rid of the `unsafe` interfacing with the C-based [htslib](https://github.com/samtools/htslib), which has [many vulnerabilities](https://github.com/samtools/htslib/pulls?q=oss-fuzz) along with other [also problematic dependencies such as OpenSSL](https://www.openssl.org/news/vulnerabilities.html). In contrast, this work uses the [independently audited RustLS counterpart](http://jbp.io/2020/06/14/rustls-audit.html) for SSL.

# Quickstart

This README assumes the following prerequisites:

1. You are already authenticated against AWS in your shell.
1. You have a [functioning Rust(up) installation](https://rustup.rs/).
1. You have adjusted the KEY, BUCKET, REGION constants in `main.rs`
1. You have installed cargo-lambda and prerequisites via `cargo install cargo-lambda`.

## Local run

Just run the following commands on **separate terminal sessions**:

```
$ cargo lambda start
$ cargo lambda invoke s3-rust-noodles-bam --data-file event.json
```

## Deployment

```
$ cdk deploy
```