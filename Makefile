build-s3Bam:
# TODO: Put conditional code depending on arch... for now it will lead to Exec error anyway though unless it's x86_64 provided.al2 :/
# Intel
#	cargo build --release --target x86_64-unknown-linux-gnu
#	cp ./target/x86_64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
#
# ARM64: Failed attempt to use rust-embedded cross to cross-compile on an M1 to the Lambda runtime environment
#	cross build --release --target x86_64-unknown-linux-gnu
#	cp ./target/x86_64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
#
# Auto (arch determined by compile host)... only works with x86_64-unknown-linux-gnu because it's what lambda runtime is in production
	cargo build --release
	cp ./target/debug/bootstrap $(ARTIFACTS_DIR)
