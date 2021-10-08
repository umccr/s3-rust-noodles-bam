build-s3Bam:
	cargo build --release --target aarch64-unknown-linux-gnu
	cp ./target/aarch64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
