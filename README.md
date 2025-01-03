# exchange-collection
> machine-readable crypto exchange OpenAPI / AsyncAPI doc and clients

[![doc](https://img.shields.io/badge/doc-rapidoc-blue)](https://repoch.co/exchange-collection)
[![license](https://img.shields.io/github/license/kanekoshoyu/exchange-collection)](https://github.com/kanekoshoyu/exchange-collection/blob/master/LICENSE)
[![discord](https://img.shields.io/discord/1153997271294283827)](https://discord.gg/q3j5MYdwnm)  


## why use machine readable API docs

#### pain point: growth of cross exchnage trading
Many trading strategies require data and execution at multiple exchanges. Trading libraries provides abstraction over multiple exchanges.

| library / framework                                    | multi-exchange | primary language | wrapper language    |
| ------------------------------------------------------ | -------------- | ---------------- | ------------------- |
| [ccxt](https://github.com/ccxt/ccxt)                   | yes            | js               | python, php, csharp |
| [hummingbot](https://github.com/hummingbot/hummingbot) | yes            | python, cpp      | n/a                 |
| [openlimits](https://github.com/nash-io/openlimits)    | yes            | rust             | python, js, go      |
| [barter-rs](https://github.com/barter-rs/barter-rs)    | yes            | rust             | n/a                 |
| [kelp](https://github.com/stellar-deprecated/kelp)     | yes            | go               | n/a                 |

They have issues in the below aspects:
1. **integration effort**: there are many crypto exchanges (N) and programming languages (L). The effort to convert those written API doc into exchange library is B, then the overall effort is N * L * B.
2. **document consistecy**: exchanges freqently update API and there is no proper versioning pipeline, causing API doc inconsistency.
3. **opinionated framework**: cross-exchange libraries often designed as complex framework,and often fails to meet the business needs.
4. **multi-language support**: generally people prefer python for proof of concept and rust for production. we should use rust as backbone, then provide python support on top, but also provide flexibility for native python vertical integration.

#### let's automate
by gathering OpenAPI / AsyncAPI docuements, we can build a CI pipeline that generate and test versioned clients for different languages solving the above issues. 
1. crowdsource both OpenAPI / AsyncAPI YAML per exchange.
2. Set up codegen CI for generating REST/WS clients.
3. Implement trading traits per generated model. I have set up trading traits in [guilder](https://github.com/kanekoshoyu/guilder). If you do not like it, feel free to still use the this repo for the OpenAPI / AsyncAPI and proprietary clients. 

## project structure
| location                       | feature                                                                             |
| ------------------------------ | ----------------------------------------------------------------------------------- |
| [asset](./asset/)              | OpenAPI and AsyncAPI YAML                                                           |
| [codegen](./codegen/README.md) | codegen script in rust, run locally to generate and push                            |
| [target](./target/README.md)   | generated code in python and rust                                                   |
| [index.html](./index.html)     | OpenAPI / AsyncAPI viewer, hosted [here](https://www.repoch.co/exchange-collection) |

## guidelines
| specs                           | guidelines                                                                                            |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| OpenAPI format                  | `{exchange}_rest_openapi.yaml`, v3.X.Y, convert swagger to OpenAPI [here](https://editor.swagger.io/) |
| AsyncAPI format                 | `{exchange}_ws_asyncapi.yaml`, YAML, v3.0.X, codegen topology natively supports V3 now                |
| codegen                         | written in Rust                                                                                       |
| official codegen output support | `rust` `python (codegen coming soon)`                                                                 |
| unofficial support              | `typescript` `csharp` `golang` `java` `dart` `kotlin` `php` `cplusplus` `scala`                       |

## codegen commands
#### initial set up
install OpenAPI CLI
```
npm install -g @openapitools/openapi-generator-cli
```
install AsyncAPI CLI
```
npm install -g @asyncapi/generator
```
#### each codegen command
| language | input                             | command                                                                                                    |
| -------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| rust     | openapi (REST, reqwest)           | `openapi-generator-cli generate -i {YAML} -g rust -o {OUTPUT_DIR} --additional-properties=library=reqwest` |
| rust     | asyncapi (WS, tokio-tungstenite)  | `asyncapi generate fromTemplate {YAML} asyncapi-rust-ws-template -p exchange={EXCHANGE}`                   |
| python   | openapi (REST, asyncio)           | `openapi-generator-cli generate -i {YAML} -g python -o {OUTPUT_DIR} --additional-properties=asyncio=true`  |
| python   | asyncapi (WS, asyncio-websockets) | `wip`                                                                                                      |


## exchange integration status
below are the list of exchanges planned for integration. Please contact me if you want to integrate for orderbook exchange.
| Exchange API                                                   | Custodial | REST (OpenAPI) | WS (AsyncAPI) |
| -------------------------------------------------------------- | --------- | -------------- | ------------- |
| [ccxtrest](https://github.com/ccxt-rest/ccxt-rest)             | /         | done           | /             |
| [hyperliquid](https://hyperliquid.gitbook.io)                  | no        | done           | done          |
| [bitwyre](https://docs.bitwyre.com)                            | yes       | done           | done          |
| [bitget](https://www.bitget.com/api-doc)                       | yes       | done           | done          |
| [binance](https://binance-docs.github.io)                      | yes       | done           | done          |
| [coinbase](https://docs.cdp.coinbase.com)                      | yes       | done           | done          |
| [hashkey](https://hashkeypro-apidoc.readme.io) (HK)            | yes       | done           | done          |
| [okx](https://www.okx.com/docs-v5/en)                          | yes       | done           | done          |
| [krakenfutures](https://docs.kraken.com/api)                   | yes       | done           | done          |
| [gateio](https://www.gate.io/docs/developers/apiv4)            | yes       | WIP            | WIP           |
| [dydx](https://docs.dydx.exchange)                             | no        | postponed      | /             |
| [polkadex](https://docs.polkadex.trade)                        | no        | postponed      | /             |
| [zkex](https://docs.zkex.com)                                  | no        | postponed      | /             |
| [gmx](https://gmx-docs.io)                                     | no        | postponed      | /             |
| [bybit](https://bybit-exchange.github.io/docs/)                | yes       | planned        | planned       |
| [kucoin](https://www.kucoin.com/docs)                          | yes       | planned        | planned       |
| [htx](https://www.htx.com/en-us/opend/newApiPages)             | yes       | planned        | planned       |
| [bitbank](https://lightning.bitflyer.com/docs) (JP)            | yes       | planned        | planned       |
| [bitflyer](https://lightning.bitflyer.com/docs) (JP)           | yes       | planned        | planned       |
| [coincheck](https://coincheck.com/documents/exchange/api) (JP) | yes       | planned        | planned       |
| [korbit](https://apidocs.korbit.co.kr) (KR)                    | yes       | planned        | planned       |
| [bitkub](https://docs.polkadex.trade) (TH)                     | yes       | planned        | planned       |




I currently have no plan of supporting [FIX protocol](https://www.fixtrading.org/what-is-fix) due to limited number of supported exchanges. But it is definitely an interesting one to try in the future.  

## TODO
- [ ] gather assets
  - [x] gather 3 sets of API docs initial assets
  - [ ] gather 10 exchanges to validate the idea
  - [ ] gather 100 exchanges to have a competitive trading library
- [ ] set up CI for codegen model
  - [ ] rust codegen
    - [x] REST (reqwest) client
    - [x] WS (tokio-tungstenite) client template 
    - [ ] CI for release on [crates.io](https://crates.io)
  - [ ] python codegen
    - [x] REST client
    - [ ] WS (asyncio-websockets)
    - [ ] CI for release on [pip]()
- [x] set up [guilder](https://github.com/kanekoshoyu/guilder) trading library
  - [x] define market data traits
  - [ ] define order placement traits
  - [ ] define ledger traits
  - [ ] implement traits on top of the codegen model
  - [ ] package models with opinionated trait per language

## notes
- the `ag` command seems to be deprecated and cannot generate code properly, use 'asyncapi generate' instead
- you can install `asyncapi-preview` extension on vs code for preview
- comnunity AsyncAPI templates like `python-sanic-template` are not working properly 

## partnership
I keep this project opensource so that everyone can take part of it. If you have any OpenAPI / AsyncAPI document for a crypto exchange, you are more than welcome to add with a pull request, or I am willing to purchase at reasonable cost as well.  
If you want to get an exchange integrated, I can help get that up for an one-off cost in one week, just enough to pay my freelancing partner to get it done.  
Please contact [Sho](https://github.com/kanekoshoyu) for partnerships.  

## recruitment
#### OpenAPI / AsyncAPI Author
I am gathering API doc with @pakTech786 would be great if more people can help with it.  
#### TypeScript AsyncAPI Template Developer
> [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template)

I have set up a repo to develop AsyncAPI template for Rust WS in React. I want a version for python as well. I am not a TS expert, so I would love to have an expert to accelarate development.  

## see also
- [guilder](https://github.com/kanekoshoyu/guilder) - Unopinionated Cross-Exchange Crypto Trading Library
- [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template) - AsyncAPI Template for Generating Rust WebSocket Client
- [kucoin-arbitrage](https://github.com/kanekoshoyu/kucoin_arbitrage) - KuCoin Cyclic Arbitrage, in Tokio Rust (legacy)
- [typed-websocket](https://github.com/kanekoshoyu/typed-websocket) - rust typed websocket client
