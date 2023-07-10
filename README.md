# COURAGEOUS Format Specification
This repository holds the code responsible for generating the JSON schema of each version of the [COURAGEOUS data format](https://grvc.us.es/courageous/),
as well as a Rust crate containing all types present in the schema, for usage in Rust applications
that require the format.

## Building the schema
Run the following with the latest stable Rust toolchain installed on your system:
```sh
cargo run --example schemagen --features schemars
```
