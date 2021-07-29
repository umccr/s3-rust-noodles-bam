# Wishlist for improvements in the Rust-AWS tooling ecosystem

The following issues could be legit or just things I don't know (yet) how to do or fix, bear with me ;)

1. Local running of lambdas is still a moving target with SAM CLI?: https://github.com/awslabs/aws-lambda-rust-runtime/pull/332#issuecomment-864435051
   1. Although there are some PoCs and workarounds from 3rd parties: https://github.com/pepoviola/tide-lambda-listener-example
1. Very slow to build and deploy a Rust Lambda (~3min `sam build` + 2min `sam deploy` on a Beta Codespaces instance) and [SAM cli is still lacking good cargo integration on the SAM tooling level](https://twitter.com/braincode/status/1371660403785142273). Ideally, from a DX standpoint, a lambda should be as easy to deploy as:
```
$ cargo aws deploy
```
Keeping the SAM-CLI `template.yml` or even more conveniently, adding a section to `Cargo.toml` that mirrors the SAM spec but the same SAM tooling runs underneath, without staying on the (Rust) way as it is now...  Or similar.
1. AWS's `sam local invoke -e event.json` to work properly, in progress via twitter: https://twitter.com/braincode/status/1375309688573599747
1. AWS's `sam local start-api` does not seem to work either, it generates some random looking binary request that is met with a HTTP 400 code as Response:
```
$ sam local start-api
Mounting s3Bam at http://127.0.0.1:3000/ [DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT]
Mounting s3Bam at http://127.0.0.1:3000/{proxy+} [DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT]
You can now browse to the above endpoints to invoke your functions. You do not need to restart/reload SAM CLI while working on your functions, changes will be reflected instantly/automatically. You only need to restart SAM CLI if you update your AWS SAM template
2021-03-27 14:12:27  * Running on http://127.0.0.1:3000/ (Press CTRL+C to quit)
2021-03-27 14:12:30 127.0.0.1 - - [27/Mar/2021 14:12:30] code 400, message Bad request version ("Ös;ìs\x9b`¢×\x90+Á\x97\x17>Xmá\x1a~~R;&lì\x908ý·\x91\x97\x00>\x13\x02\x13\x03\x13\x01À,À0\x00\x9fÌ©Ì¨ÌªÀ+À/\x00\x9eÀ$À(\x00kÀ#À'\x00gÀ")
¢×>Xmá~~R;&lì·>À,À0©Ì¨ÌªÀ+À/$À(kÀ#À'gÀ" HTTPStatus.BAD_REQUEST -Õ]å[oómÚÞE0¹]R£E+X× Ös;ìs
2021-03-27 14:12:30 127.0.0.1 - - [27/Mar/2021 14:12:30] code 400, message Bad request version ("Ô\x86S\x00>\x13\x02\x13\x03\x13\x01À,À0\x00\x9fÌ©Ì¨ÌªÀ+À/\x00\x9eÀ$À(\x00kÀ#À'\x00gÀ")
2021-03-27 14:12:30 127.0.0.1 - - [27/Mar/2021 14:12:30] "2.Û¼/æI4Í2À¡¿â
\kÁ3Á q}F a@é ½FÇÙLäÊÕåÚ°zm3?$NÖÂø ÔS>À,À0©Ì¨ÌªÀ+À/$À(kÀ#À'gÀ" HTTPStatus.BAD_REQUEST -
```
1. Cannot cross-compile `ring` dependency on an Apple Silicon while still being able to deploy on AWS (x86_64-unknown_linux_gnu), see error:

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
