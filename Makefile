build-s3Bam:
	cargo build --release --target aarch64-unknown-linux-gnu
	#docker build -t provided.al2-rust . -f Dockerfile-provided.al2
	#sam build -c -u --skip-pull-image -bi provided.al2-rust
	cp ./target/aarch64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
