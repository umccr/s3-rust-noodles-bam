# Read BAM header on an AWS lambda with Noodles

This is a small Bioinformatics proof of concept that bundles [noodles](http://github.com/zaeleus/noodles) into an AWS Lambda.

A previous lambda was written using the C-bindgen-based [rust-htslib](https://github.com/brainstorm/s3-rust-htslib-bam). This iteration gets rid of the `unsafe` interfacing with the C-based [htslib](https://github.com/samtools/htslib), which has [many vulnerabilities](https://github.com/samtools/htslib/pulls?q=oss-fuzz) along with other [also problematic dependencies such as OpenSSL](https://www.openssl.org/news/vulnerabilities.html). In contrast, this work uses the [independently audited RustLS counterpart](http://jbp.io/2020/06/14/rustls-audit.html) for SSL.

# Quickstart

This README assumes the following prerequisites:

1. You are already authenticated against AWS in your shell.
2. You have a [functioning Rust(up) installation](https://rustup.rs/).
3. You have adjusted the KEY, BUCKET, REGION constants in `main.rs`
4. You have installed cargo-lambda and prerequisites via `cargo install cargo-lambda`.
5. You should prepare [small test BAM](https://github.com/umccr/ega-submit/tree/master/test) in some S3 bucket. See `event.json`

## Local Run

Just run the following commands on **separate terminal sessions**.

Build Local:
```
cargo build
```

Start Local Server:
```
cargo lambda start
```

Invoke Lambda with event or by pointing it to BAM file:
```
cargo lambda invoke s3-rust-noodles-bam --data-file events/event.json | jq
cargo lambda invoke s3-rust-noodles-bam --data-ascii '{"bam": "s3://some/key.bam"}' | jq
```

Invoke Http Lambda with APIGateway mock event:

```
cargo lambda invoke apigw --data-file events/mock_event.json | jq
cargo lambda invoke apigw --data-file events/mock_event_big.json | jq
cargo lambda invoke apigw --data-file events/mock_event_empty.json | jq
```

## Deployment

Install CDK dependencies:

```
npm install cdk
```

Build fresh and deploy:
```
cargo clean
cdk diff
cdk deploy
```


## Testing

Call Main Lambda Function:
```
aws lambda invoke --function-name <s3-rust-noodles-bam-...> out.json
```

> NOTE: `awscurl` requires AWS credentials

Call Endpoint:
```
awscurl -H "Accept: application/json" --region ap-southeast-2 "https://<my-apigw-ep>.execute-api.ap-southeast-2.amazonaws.com/prod/" | jq
```

Call Endpoint with POST payload:
```
awscurl -X POST -d "@event_big.json" -H "Content-Type: application/json" --region ap-southeast-2 "https://<my-apigw-ep>.execute-api.ap-southeast-2.amazonaws.com/prod/" | jq
```
