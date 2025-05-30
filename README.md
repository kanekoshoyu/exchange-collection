# exchange-collection
> list of machine-readable crypto exchange OpenAPI AsyncAPI documents and code generators. 

> [!IMPORTANT]  
> generated code is now ready at [exchange-collection-rust](https://github.com/kanekoshoyu/exchange-collection-rust)

[![doc](https://img.shields.io/badge/doc-rapidoc-blue)](https://repoch.co/exchange-collection)
[![license](https://img.shields.io/github/license/kanekoshoyu/exchange-collection)](https://github.com/kanekoshoyu/exchange-collection/blob/master/LICENSE)
[![discord](https://img.shields.io/discord/1153997271294283827)](https://discord.gg/q3j5MYdwnm)  


## why use OpenAPI AsyncAPI docs

### pain points in cross exchange trading
Many trading strategies require cross exchange data and execution. There are trading libraries that provides abstraction across exchanges.

| library / framework                                    | multi-exchange | primary language | wrapper language    |
| ------------------------------------------------------ | -------------- | ---------------- | ------------------- |
| [ccxt](https://github.com/ccxt/ccxt)                   | yes            | js               | python, php, csharp |
| [hummingbot](https://github.com/hummingbot/hummingbot) | yes            | python, cpp      | n/a                 |
| [openlimits](https://github.com/nash-io/openlimits)    | yes            | rust             | python, js, go      |
| [barter-rs](https://github.com/barter-rs/barter-rs)    | yes            | rust             | n/a                 |
| [kelp](https://github.com/stellar-deprecated/kelp)     | yes            | go               | n/a                 |

These libraries are each good in their way but does not solve the below 4 points at once.
1. **integration effort**: N crypto exchanges, L programming languages, its a better when effort is N + L rather than N * L
2. **document consistecy and code maintenance**: exchanges freqently update API. Without proper pipeline and versioning, code and API inconsistency is common
3. **opinionated framework**: battery-included cross-exchange libraries has complex design, and often fail to meet the actual needs in low latency
4. **multi-language support**: people use python for strategy proof of concept and rust for production.

#### solution: building from bottom up
by gathering OpenAPI / AsyncAPI docuements, we can build a CI pipeline that generate and test versioned clients for different languages solving the above issues. 
1. gather both OpenAPI / AsyncAPI YAML per exchange
2. develop codegen CI for generating REST/WS clients
3. deploy CI for generating REST/WS clients
4. implement trading traits per generated model
   1. I have set up trading traits in [guilder](https://github.com/kanekoshoyu/guilder) for generated codes
   2. or you can still use the this repo for the OpenAPI / AsyncAPI  

## project structure
| location                       | feature                                                                             |
| ------------------------------ | ----------------------------------------------------------------------------------- |
| [asset](./asset/)              | OpenAPI and AsyncAPI YAML                                                           |
| [codegen](./codegen/README.md) | code generator script written in rust                                               |
| [index.html](./index.html)     | OpenAPI / AsyncAPI viewer, hosted [here](https://www.repoch.co/exchange-collection) |

this repo includes a CI which generates rust client and pushes to [exchange-collection-rust](https://github.com/kanekoshoyu/exchange-collection-rust)

## guidelines
| specs                           | guidelines                                                                                            |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| OpenAPI format                  | `{exchange}_rest_openapi.yaml`, v3.X.Y, convert swagger to OpenAPI [here](https://editor.swagger.io/) |
| AsyncAPI format                 | `{exchange}_ws_asyncapi.yaml`, YAML, v3.0.X, codegen topology natively supports V3 now                |
| codegen                         | written in Rust                                                                                       |
| official codegen output support | `rust` `python (codegen coming soon)`                                                                 |
| unofficial support              | `typescript` `csharp` `golang` `java` `dart` `kotlin` `php` `cplusplus` `scala`                       |

## exchange integration status
- [list of notable exchanges](https://coinmarketcap.com/rankings/exchanges/derivatives)
- [list of no kyc exchanges](https://koinly.io/blog/top-no-kyc-crypto-exchanges)

Below is a the list of exchanges planned for integration. Please contact [Sho](https://github.com/kanekoshoyu) if you want to request exchanges for integration.

| Exchange API | KYC | REST (OpenAPI) | WS (AsyncAPI) | Beeceptor Mock Server |
|--------------|-----|----------------|----------------|------------------------|
| [ccxtrest](https://github.com/ccxt-rest/ccxt-rest) | / | done | / | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/ccxtrest_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [hyperliquid](https://hyperliquid.gitbook.io) | no | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/hyperliquid_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [mexc](https://www.mexc.com/mexc-api) | no | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/mexc_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [bitwyre](https://docs.bitwyre.com) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/bitwyre_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [bitget](https://www.bitget.com/api-doc) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/bitget_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [binance](https://binance-docs.github.io) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/binance_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [coinbase](https://docs.cdp.coinbase.com) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/coinbase_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [coincheck](https://coincheck.com/documents/exchange/api) (JP) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/coincheck_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [hashkey](https://hashkeypro-apidoc.readme.io) (HK) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/hashkey_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [okx](https://www.okx.com/docs-v5/en) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/okx_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [krakenfutures](https://docs.kraken.com/api) | yes | done | done | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/krakenfutures_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [gateio](https://www.gate.io/docs/developers/apiv4) | yes | WIP | WIP |  |
| [dydx](https://docs.dydx.exchange) | no | postponed | / |  |
| [polkadex](https://docs.polkadex.trade) | no | postponed | / |  |
| [zkex](https://docs.zkex.com) | no | postponed | / |  |
| [gmx](https://gmx-docs.io) | no | postponed | / |  |
| [coinex](https://docs.coinex.com/api/v2) | no | WIP | WIP |  |
| [bybit](https://bybit-exchange.github.io/docs) | yes | planned | planned | <a href="https://beeceptor.com/openapi-mock-server/?utm_source=github-exchange-collection&url=https://raw.githubusercontent.com/kanekoshoyu/exchange-collection/refs/heads/main/asset/bybit_rest_openapi.yaml" target="_blank"><img src="https://cdn.beeceptor.com/assets/images/buttons/mock-openapi-with-beeceptor.png" alt="Mock These APIs Instantly" style="height: 60px;"></a> |
| [kucoin](https://www.kucoin.com/docs) | yes | planned | planned |  |
| [htx](https://www.htx.com/en-us/opend/newApiPages) | yes | planned | planned |  |
| [bitbank](https://lightning.bitflyer.com/docs) (JP) | yes | postponed | postponed |  |
| [bitflyer](https://lightning.bitflyer.com/docs) (JP) | yes | postponed | postponed |  |
| [korbit](https://apidocs.korbit.co.kr) (KR) | yes | postponed | postponed |  |
| [bitkub](https://docs.polkadex.trade) (TH) | yes | postponed | postponed |  |


- I currently have no plan of supporting [FIX protocol](https://www.fixtrading.org/what-is-fix) due to limited number of supported exchanges. Once the project reaches certain maturity it will be implemented.  
- For AMM DEX like [Uniswap](https://docs.uniswap.org/contracts/v2/reference/API/overview), I will add support if there is a proper demand. My suggestion is to make use of JSON-RPC clients like ether-rs to connect RPC providers like [infura](https://www.infura.io) and [alchemy](https://www.alchemy.com). 

## codegen status
| Document | Language | framework         | CI status  | code issues                                                              |
| -------- | -------- | ----------------- | ---------- | ------------------------------------------------------------------------ |
| OpenAPI  | rust     | reqwest           | integrated | generated code cannot differentiate fields like "m" and "M"              |
| AsyncAPI | rust     | tokio-tungstenite | integrated | does not parse variables in URL/channel name like "price_{base}_{quote}" |
| AsyncAPI | rust     | async-tungstenite | planned    | /                                                                        |
| OpenAPI  | python   | reqwest           | planned    | /                                                                        |
| AsyncAPI | rust     | tokio-tungstenite | planned    | /                                                                        |

### codegen commands used
you can use these commands to generate code manually as well
#### initial set up
install OpenAPI CLI
```
npm install -g @openapitools/openapi-generator-cli
```
install AsyncAPI CLI
```
npm install -g @asyncapi/cli
```
#### each codegen command
| language | input                             | command                                                                                                    |
| -------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| rust     | openapi (REST, reqwest)           | `openapi-generator-cli generate -i {YAML} -g rust -o {OUTPUT_DIR} --additional-properties=library=reqwest` |
| rust     | asyncapi (WS, tokio-tungstenite)  | `asyncapi generate fromTemplate {YAML} asyncapi-rust-ws-template -p exchange={EXCHANGE}`                   |
| python   | openapi (REST, asyncio)           | `openapi-generator-cli generate -i {YAML} -g python -o {OUTPUT_DIR} --additional-properties=asyncio=true`  |
| python   | asyncapi (WS, asyncio-websockets) | `wip`                                                                                                      |

## TODO
- [ ] gather assets
  - [x] gather 3 sets of API docs initial assets
  - [x] gather 10 exchanges to validate the idea
  - [ ] gather 100 exchanges to have a competitive trading library
- [ ] set up CI for codegen model
  - [ ] rust codegen
    - [x] REST (reqwest) client
    - [x] WS (tokio-tungstenite) client template 
    - [x] CI for pushing to sub-repo on generated code
    - [ ] CI for release on [crates.io](https://crates.io)
  - [ ] python codegen
    - [x] REST client
    - [ ] WS (asyncio-websockets)
    - [ ] CI for pushing to sub-repo on generated code
    - [ ] CI for release on [pip]()
- [x] set up [guilder](https://github.com/kanekoshoyu/guilder) trading library
  - [x] define market data traits
  - [ ] define order placement traits
  - [ ] define ledger traits
  - [ ] implement traits on top of the codegen model
  - [ ] package models with opinionated trait per language

## notes
- the `ag` command seems to be deprecated and cannot generate code properly, use `asyncapi generate` instead
- you can install `asyncapi-preview` extension on vs code for preview
- comnunity AsyncAPI templates like `python-sanic-template` are not working properly 

## partnership & recruitment
I keep this project opensource so that everyone can take part of it. If you have any OpenAPI / AsyncAPI document for a crypto exchange, you are more than welcome to add with a pull request, or I am willing to purchase at reasonable cost as well.  
Please contact [Sho](https://github.com/kanekoshoyu) for partnerships.  
#### OpenAPI / AsyncAPI Author
I am gathering API doc with @pakTech786 would be great if more people can help with it.  
#### TypeScript AsyncAPI Template Developer
> [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template)

I have set up a repo to develop AsyncAPI template for Rust WS in React. I want a version for python/go as well. I am not a TS expert, so I would love to have an expert to accelarate development.  

## see also
- [exchange-collection-rust](https://github.com/kanekoshoyu/exchange-collection-rust) - rust client generated from exchange-collection
- [guilder](https://github.com/kanekoshoyu/guilder) - Unopinionated Cross-Exchange Crypto Trading Library
- [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template) - AsyncAPI Template for Generating Rust WebSocket Client
- [kucoin-arbitrage](https://github.com/kanekoshoyu/kucoin_arbitrage) - KuCoin Cyclic Arbitrage, in Tokio Rust (legacy)
- [typed-websocket](https://github.com/kanekoshoyu/typed-websocket) - rust typed websocket client
