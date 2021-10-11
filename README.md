# Read BAM header on an AWS lambda with Noodles

This is a small Bioinformatics proof of concept that bundles [noodles](http://github.com/zaeleus/noodles) into an AWS Lambda.

A previous lambda was written using the C-bindgen-based [rust-htslib](https://github.com/brainstorm/s3-rust-htslib-bam). This iteration gets rid of the `unsafe` interfacing with the C-based [htslib](https://github.com/samtools/htslib), which has [many vulnerabilities](https://github.com/samtools/htslib/pulls?q=oss-fuzz) along with other [also problematic dependencies such as OpenSSL](https://www.openssl.org/news/vulnerabilities.html). In contrast, this work uses the [independently audited RustLS counterpart](http://jbp.io/2020/06/14/rustls-audit.html) for SSL.

# Quickstart

This README assumes the following prerequisites:

1. You are already authenticated against AWS in your shell.
1. [AWS SAM](https://aws.amazon.com/serverless/sam/) is properly installed.
1. You have a [functioning Rust(up) installation](https://rustup.rs/).
1. You have adjusted the KEY, BUCKET, REGION constants in `main.rs`

Building and deploying the Rust lambda on ARM64 (Graviton2 instances) can be done via [SAM-CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html). But first you must build a docker image and build this example using that container based on upstream's `public.ecr.aws/sam/build-provided.al2:latest`:

```
$ docker build -t provided.al2-rust . -f Dockerfile-provided.al2
$ sam build -c -u --skip-pull-image -bi provided.al2-rust
$ sam deploy
```

Debugging locally works but there are some credential provider management left to address:

```
% sam local start-api
Mounting s3Bam at http://127.0.0.1:3000/{proxy+} [DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT]
Mounting s3Bam at http://127.0.0.1:3000/ [DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT]
You can now browse to the above endpoints to invoke your functions. You do not need to restart/reload SAM CLI while working on your functions, changes will be reflected instantly/automatically. You only need to restart SAM CLI if you update your AWS SAM template
2021-10-11 15:25:29  * Running on http://127.0.0.1:3000/ (Press CTRL+C to quit)
Invoking bootstrap (provided.al2)
Skip pulling image and use local one: public.ecr.aws/sam/emulation-provided.al2:rapid-1.33.0-arm64.

Mounting /Users/rvalls/dev/umccr/s3-rust-noodles-bam/.aws-sam/build/s3Bam as /var/task:ro,delegated inside runtime container
START RequestId: 9b14bf12-6ef7-4e95-b98c-c7821ba00509 Version: $LATEST
END RequestId: 9b14bf12-6ef7-4e95-b98c-c7821ba00509
REPORT RequestId: 9b14bf12-6ef7-4e95-b98c-c7821ba00509  Init Duration: 1.31 ms  Duration: 185.67 ms     Billed Duration: 200 ms Memory Size: 128 MB     Max Memory Used: 128 MB
Lambda returned empty body!
Invalid lambda response received: Invalid API Gateway Response Keys: {'errorType', 'errorMessage'} in {'errorType': '&alloc::boxed::Box<dyn std::error::Error+core::marker::Send+core::marker::Sync>', 'errorMessage': 'failed to construct request: No credentials in the property bag'}
2021-10-11 15:25:39 127.0.0.1 - - [11/Oct/2021 15:25:39] "GET / HTTP/1.1" 502 -
```

Then actually call the lambda through the API Gateway in production (found easily on API Gateway's dashboard):

```
curl https://api.gateway.<RANDOM_AWS_ID>.domain.amazon.com/Prod
```

If successful, you should see the header records from the BAM file in CloudWatch and the output of API Gateway JSON response:

```
END RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f
REPORT RequestId: dbd528c7-858d-15e7-6067-9723ce1e643f  Init Duration: 139.11 ms        Duration: 2251.32 ms    Billed Duration: 2300 ms      M
emory Size: 128 MB      Max Memory Used: 13 MB

[
(...)
"@SQ\tSN:HLA-DRB1*04:03:01\tLN:15246\tAS:GRCh38\tM5:ce0de8afd561fb1fb0d3acce94386a27\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:01\tLN:16110\tAS:GRCh38\tM5:4063054a8189fbc81248b0f37b8273fd\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@SQ\tSN:HLA-DRB1*07:01:01:02\tLN:16120\tAS:GRCh38\tM5:a4b1a49cfe8fb2c98c178c02b6c64ed4\tUR:ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/GRCh38_full_analysis_set_plus_decoy_hla.fa\tSP:Human",
"@CO\t$known_indels_file(s) = ftp://ftp.1000genomes.ebi.ac.uk/vol1/ftp/technical/reference/GRCh38_reference_genome/other_mapping_resources/ALL.wgs.1000G_phase3.GRCh38.ncbi_remapper.20150424.shapeit2_indels.vcf.gz",
"@CO\tFASTQ=ERR009378_1.fastq.gz",
(...)
```
