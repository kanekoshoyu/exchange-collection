# [exchange-collection](../README.md)-codegen
codegen for exchange collection

## TODO
- [ ] <document_version>
  - [x] get version
  - [ ] sum up both asyncapi and openapi version
- [ ] <overall_version>
  - [ ] sum up all exchange version

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
