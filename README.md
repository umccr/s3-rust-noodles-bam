# Read BAM header on an AWS lambda with Noodles

This is a small Bioinformatics proof of concept that bundles [noodles](http://github.com/zaeleus/noodles) into an AWS Lambda.

A previous lambda was written using the C-bindgen-based [rust-htslib](https://github.com/brainstorm/s3-rust-htslib-bam). This iteration gets rid of the `unsafe` interfacing with the C-based [htslib](https://github.com/samtools/htslib) along with other [problematic dependencies such as OpenSSL](https://www.openssl.org/news/vulnerabilities.html) (in favour of the [independently audited RustLS counterpart](http://jbp.io/2020/06/14/rustls-audit.html)).

# Quickstart

This README assumes the following prerequisites:

1. You are already authenticated against AWS (with either environment credentials or AWS_PROFILE set) - in an
     account that you can deploy CloudFormation stacks/lambdas.
2. [AWS SAM](https://aws.amazon.com/serverless/sam/) is properly installed.
3. You have a [functioning Rust(up) installation](https://rustup.rs/).
4. You have adjusted the KEY, BUCKET, REGION constants in `main.rs`

Building and deploying the Rust lambda can be done via [SAM-CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html):

```
$ sam build && sam deploy
```

Then actually call the lambda through the API Gateway (found easily on API Gateway's dashboard):

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
