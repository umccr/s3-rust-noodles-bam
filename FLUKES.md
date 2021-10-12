# Wishlist for improvements in the Rust-AWS tooling ecosystem

The following issues could be legit or just things I don't know (yet) how to do or fix, bear with me ;)

1. Just support [Cargo instead of SAM template.yaml, Makefile, etc... hacks](https://github.com/aws-samples/serverless-rust-demo/issues/4).
2. Apple Silicon hosts [require a special docker container built](https://github.com/umccr/s3-rust-noodles-bam/blob/master/Dockerfile-provided.al2). Subsequent builds of your lambda will [**take around 6 minutes on an M1**](https://github.com/aws/aws-sam-build-images/pull/26#issuecomment-938364194). Ideally `cargo build --target aarch64-unknown-linux-gnu should just work, but [there's the ring dependency that ruins this DX](https://github.com/briansmith/ring/issues/1332). 
