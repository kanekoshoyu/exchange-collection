# [exchange-collection](../README.md)-codegen
codegen for exchange collection

## TODO (take bottom up approach)
- [ ] protocol crate (e.g.  target/rust/src/binance/src/rest)
  - [x] target/rust/src/binance/src/rest/src/[api/model]
  - [x] target/rust/src/binance/src/rest/Cargo.toml
    - [x] exchange-collection-binance-rest
    - [ ] version = "0.1.0" // from the version in the doc, current one is wrong
- [ ] exchange crate (e.g. target/rust/src/binance)
  - [ ] target/rust/src/binance/Cargo.toml // based on the generated protocols
    - [ ] name = "exchange-collection-binance"
    - [ ] version = "0.2.0" // sum of protocols under the exchange
    - [ ] exchange-collection-binance-rest = { path = "rest", version = "0.1.0" }
  - [x] target/rust/src/binance/src/lib.rs
    - [x] pub mod rest; (has to be generative, append when missing)
- [ ] collection crate (e.g. target/rust)
  - target/rust/Cargo.toml
  - name = "exchange-collection-binance"
  - version = "document_version" // sum of exchanges under the collection