# [exchange-collection](../README.md)-codegen
codegen for exchange collection

## TODO
- [ ] <module_version>
  - [x] get version
  - [ ] sum up both asyncapi and openapi version (e.g. binance WS)
- [ ] <package_version>
  - [ ] sum up all exchange version for a single package (e.g. binance)
- [ ] <collection_version>
  - [ ] sum up all exchange version for a single module (i.e. exchange-collection)

- [ ] target/rust/Cargo.toml
  - [ ] version = "<overall_version>"
  - [ ] exchange-collection-<exchange> = { path = "<exchange>", version = "<document_version>" }

- target/rust/<exchage>
- target/rust/<exchage>/Cargo.toml
  - name = "exchange-collection-<exchange>"
  - version = "document_version"
- [x] target/rust/<exchange>/src/rest/
- [x] target/rust/<exchange>/src/rest/apis
- [x] target/rust/<exchange>/src/rest/models
- [x] target/rust/<exchange>/src/rest/lib
- [x] target/rust/<exchange>/src/ws/models
- [ ] target/rust/<exchange>/src/lib.rs
  - [ ] pub use ws; (has to be genearative, append when missing)
  - [ ] pub use rest; (has to be genearative, append when missing)