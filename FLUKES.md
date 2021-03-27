# Wishlist for improvements in the Rust-AWS tooling ecosystem

1. Rusoto's [poor performance](https://twitter.com/braincode/status/1375329288732307457) and [precarious maintainership status](https://github.com/rusoto/rusoto/issues/1651)... I am hopeful that [it gets solved soon when AWS hires a dedicated SDK maintainer](https://twitter.com/braincode/status/1371648129154490368)?
1. Very slow to build and deploy a Rust Lambda (~3min `sam build` + 2min `sam deploy` on a Beta Codespaces instance) and [SAM cli is still lacking good cargo integration on the SAM tooling level](https://twitter.com/braincode/status/1371660403785142273).
1. [Lack of public Rust-lambda benchmarks](https://twitter.com/robertohuertasm/status/1368991014606757891)... with provided.al2 and with/without [jemallocator](https://lib.rs/crates/jemallocator) activated... BUT `jemalloc` is still not supported on Apple Silicon, so not convenient to work on locally (unless parametrized by `#cfg` or feature flags).
1. Cannot define a simple S3ReadOnly role on SAM's template.yml itself without pre-creating it through the web console or aws cli?: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification-generated-resources-function.html#sam-specification-generated-resources-function-not-role 
1. AWS's `sam invoke -e event.json` to work properly, in progress via twitter: https://twitter.com/braincode/status/1375309688573599747
1. Cannot cross-compile `ring` dependency on an Apple Silicon mac (perhaps related to https://gcc.gnu.org/bugzilla/show_bug.cgi?id=21098):

```rust
% cargo build --release --target x86_64-unknown-linux-gnu
(...)
error: failed to run custom build command for `ring v0.16.20`

Caused by:
  process didn't exit successfully: `/Users/rvalls/dev/umccr/s3-rust-noodles-bam/target/release/build/ring-409950ed8e3b17f6/build-script-build` (exit code: 101)
  --- stdout
  OPT_LEVEL = Some("3")
  TARGET = Some("x86_64-unknown-linux-gnu")
  HOST = Some("aarch64-apple-darwin")
  CC_x86_64-unknown-linux-gnu = None
  CC_x86_64_unknown_linux_gnu = None
  TARGET_CC = None
  CC = None
  CROSS_COMPILE = None
  CFLAGS_x86_64-unknown-linux-gnu = None
  CFLAGS_x86_64_unknown_linux_gnu = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")

  --- stderr
  running "cc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I" "include" "-Wall" "-Wextra" "-pedantic" "-pedantic-errors" "-Wall" "-Wextra" "-Wcast-align" "-Wcast-qual" "-Wconversion" "-Wenum-compare" "-Wfloat-equal" "-Wformat=2" "-Winline" "-Winvalid-pch" "-Wmissing-field-initializers" "-Wmissing-include-dirs" "-Wredundant-decls" "-Wshadow" "-Wsign-compare" "-Wsign-conversion" "-Wundef" "-Wuninitialized" "-Wwrite-strings" "-fno-strict-aliasing" "-fvisibility=hidden" "-fstack-protector" "-g3" "-DNDEBUG" "-c" "-o/Users/rvalls/dev/umccr/s3-rust-noodles-bam/target/x86_64-unknown-linux-gnu/release/build/ring-7d583ca99ada65cc/out/aesni-x86_64-elf.o" "/Users/rvalls/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/pregenerated/aesni-x86_64-elf.S"
  /Users/rvalls/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/pregenerated/aesni-x86_64-elf.S:1181:19: error: unexpected token in '.section' directive
  .section .note.GNU-stack,"",@progbits
                    ^
  thread 'main' panicked at 'execution failed', /Users/rvalls/.cargo/registry/src/github.com-1ecc6299db9ec823/ring-0.16.20/build.rs:656:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
```