# [exchange-collection](../README.md)-codegen
> codegen for exchange collection

[changelog](./CHANGELOG.md)
## TODO (take bottom up approach)
- [ ] come up with a complete guideline on nested semantic versioning
- [ ] protocol crate (e.g.  target/rust/src/binance/src/rest)
  - [ ] target/rust/src/binance/src/rest/src/model (linting fails in the generated code)
  - [x] target/rust/src/binance/src/rest/Cargo.toml
    - [x] exchange-collection-binance-rest
    - [x] version = "0.1.0"
    - [x] dependencies (serde, serde_repr, serde_json, reqwest, url)
- [ ] exchange crate (e.g. target/rust/src/binance)
  - [x] target/rust/src/binance/Cargo.toml // based on the generated protocols
    - [x] name = "exchange-collection-binance"
    - [ ] version = "0.2.0" // sum of protocols under the exchange
    - [x] exchange-collection-binance-rest = { path = "rest", version = "0.1.0" }
  - [x] target/rust/src/binance/src/lib.rs
    - [x] pub mod rest; (has to be generative, append when missing)
- [ ] collection crate (e.g. target/rust)
  - [ ] target/rust/Cargo.toml
    - [x] name = "exchange-collection"
    - [ ] version = "document_version" // sum of exchanges under the collection
    - [ ] exchange-collection-binance = { path = "binance", version = "0.1.0" }
