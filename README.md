# Read BAM header on an AWS lambda with Noodles

This is a small Bioinformatics proof of concept that bundles [noodles](http://github.com/zaeleus/noodles) into an AWS Lambda.

A previous lambda was written using the C-bindgen-based [rust-htslib](https://github.com/brainstorm/s3-rust-htslib-bam). This iteration gets rid of the `unsafe` interfacing with the C-based [htslib](https://github.com/samtools/htslib), which has [many vulnerabilities](https://github.com/samtools/htslib/pulls?q=oss-fuzz) along with other [also problematic dependencies such as OpenSSL](https://www.openssl.org/news/vulnerabilities.html). In contrast, this work uses the [independently audited RustLS counterpart](http://jbp.io/2020/06/14/rustls-audit.html) for SSL.

# Quickstart

This README assumes the following prerequisites:

1. You are already authenticated against AWS in your shell.
1. [AWS SAM][sam] is properly installed.
1. You have a [functioning Rust(up) installation](https://rustup.rs/).
1. You have adjusted the KEY, BUCKET, REGION constants in `main.rs`
1. You have installed cargo-lambda and prerequisites via `cargo install cargo-lambda`.

## Local run

There's currently two supported ways to run the lambda locally: using [SAM][sam] and using [cargo-lambda][cargo-lambda].

### With cargo-lambda

Just run the following commands on **separate terminal sessions**:

```
$ cargo lambda start
$ cargo lambda invoke s3-rust-noodles-bam --data-file event.json
```

## Building and running Lambda in Docker

Building and deploying the Rust lambda on ARM64 (Graviton2 instances) can be done via [SAM-CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html). But first you must build a docker image and build this example using that container based on upstream's `public.ecr.aws/sam/build-provided.al2:latest`:

```
$ docker build -t provided.al2-rust . -f Dockerfile-provided.al2
$ sam build -c -u --skip-pull-image -bi provided.al2-rust
$ sam deploy
```

### With SAM

Debugging locally also works using `sam local start-api`.

Then one can actually invoke the lambda through the API Gateway in production (found easily on API Gateway's dashboard):

```
curl https://api.gateway.<RANDOM_AWS_ID>.domain.amazon.com/Prod
```

If successful, you should see the header records with `curl`, i.e with `sam local start-api` running locally:

```
$ curl http://127.0.0.1:3000/
(...)
"@SQ\tSN:HLA-DRB1*04:03:01\tLN:15246\tAS:GRCh38\tM5:ce0de8afd561fb1fb0d3acce94386a27\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:01\tLN:16110\tAS:GRCh38\tM5:4063054a8189fbc81248b0f37b8273fd\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:02\tLN:16120\tAS:GRCh38\tM5:a4b1a49cfe8fb2c98c178c02b6c64ed4\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@CO\t$known_indels_file(s) = ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/other_mapping_resources/ALL.wgs.1000G_phase3.GRCh38.ncbi_remapper.20150424.shapeit2_indels.vcf.gz",
"@CO\tFASTQ=ERR009378_1.fastq.gz",
(...)
```

[sam]: https://aws.amazon.com/serverless/sam/
[cargo-lambda]: https://crates.io/crates/cargo-lambda
