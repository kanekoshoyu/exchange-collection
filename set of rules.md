set of rules

- <document_version>
  - sum up both asyncapi and openapi version

- target/rust/Cargo.toml
  - exchange-collection-<exchange> = { path = "<exchange>", version = "<document_version>" }

- target/rust/<exchage>
- target/rust/<exchage>/Cargo.toml
  - name = "exchange-collection-<exchange>"
  - version = "document_version"

- target/rust/<exchange>/src/rest/
- target/rust/<exchange>/src/rest/apis
- target/rust/<exchange>/src/rest/models
- target/rust/<exchange>/src/rest/lib
- target/rust/<exchange>/src/ws/models
