# Wishlist for improvements in the Rust-AWS tooling ecosystem

The following issues could be legit or just things I don't know (yet) how to do or fix, bear with me ;)

1. Just support [Cargo instead of SAM template.yaml, Makefile, etc... hacks](https://github.com/aws/aws-lambda-builders/pull/174).
1. AWS's `sam local invoke -e event.json` to work properly, in progress via twitter: https://twitter.com/braincode/status/1375309688573599747 ... not a problem anymore for Rust lambda runtime, `sam local start-api` works fine.
1. Cannot cross-compile `ring` dependency on an Apple Silicon while still being able to deploy on AWS (x86_64-unknown_linux_gnu), see issues:
  1.1 [ring issue](https://github.com/briansmith/ring/issues/1332).
  1.2 https://github.com/aws/aws-sam-cli/issues/3132
  1.3 https://github.com/aws/aws-sam-cli-app-templates/pull/129
