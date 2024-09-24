# exchange-collection
> crypto orderbook exchange OpenAPI(REST) and AsyncAPI(WS) doc and clients

[![doc](https://img.shields.io/badge/doc-rapidoc-blue)](https://repoch.co/exchange-collection)
[![license](https://img.shields.io/github/license/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/blob/master/LICENSE)
[![ci](https://img.shields.io/github/actions/workflow/status/kanekoshoyu/kucoin_arbitrage/rust.yml)](https://github.com/kanekoshoyu/kucoin_arbitrage/actions)
[![issues](https://img.shields.io/github/issues/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/issues)
[![discussions](https://img.shields.io/github/discussions/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/discussions)
[![discord](https://img.shields.io/discord/1153997271294283827)](https://discord.gg/q3j5MYdwnm)  

## problem with other cross-exchange libraries/frameworks
Most of the trading strategies require data and execution at different exchanges. There are trading libraries that provides abstraction over multiple exchanges.

| library / framework                                    | multi-exchange | primary language | wrapper language    |
| ------------------------------------------------------ | -------------- | ---------------- | ------------------- |
| [ccxt](https://github.com/ccxt/ccxt)                   | yes            | js               | python, php, csharp |
| [hummingbot](https://github.com/hummingbot/hummingbot) | yes            | python, cpp      | n/a                 |
| [openlimits](https://github.com/nash-io/openlimits)    | yes            | rust             | python, js, go      |
| [barter-rs](https://github.com/barter-rs/barter-rs)    | yes            | rust             | n/a                 |
| [kelp](https://github.com/stellar-deprecated/kelp)     | yes            | rust             | n/a                 |

They have issues in the below aspects:
1. **marginal integration effort**: there are many crypto exchanges (N) and programming languages (L). The effort to convert those written API doc into exchange library is B, then the overall effort is N * L * B.
2. **document consistecy**: exchanges freqently update API and there is no proper versioning pipeline, causing API doc inconsistency.
3. **opinionated framework**: cross-exchange libraries often designed as complex framework,and often fails to meet the business needs.
4. **multi-language support**: generally people prefer python for proof of concept and rust for production. we should use rust as backbone, then provide python support on top, but also provide flexibility for native python vertical integration.

## proposal
I propose a streamlined integration of exchange API with a new approach, by using machine-readable API documents; OpenAPI for REST and AsyncAPI for WebSocket.
1. gather both OpenAPI and AsyncAPI YAML per exchange (with the collective help by freelancers)
2. Set up codegen CI for generating REST/WS clients.
3. Implement trading traits per generated model. I have set up trading traits in [guilder](https://github.com/kanekoshoyu/guilder). If you do not like it, feel free to still use the this repo for the OpenAPI / AsyncAPI and its code-gen clients. 

## structure
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
| AsyncAPI format                 | `{exchange}_ws_asyncapi.yaml`, YAML, v2.X.Y, codegen does not work well with v3 apparently            |
| codegen                         | Rust                                                                                                  |
| official codegen output support | `rust` `python`                                                                                       |
| unofficial support              | `typescript` `csharp` `golang` `java` `dart` `kotlin` `php` `cplusplus` `scala`                       |

## codegen commands
### initial set up
install OpenAPI CLI
```
npm install -g @openapitools/openapi-generator-cli
```
install AsyncAPI CLI
```
npm install -g @asyncapi/generator
```
### OpenAPI model
```
openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
```
### OpenAPI rust client (REST, reqwest)
```
openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model --additional-properties=library=reqwest
```
### OpenAPI python client (REST, asyncio)
```
openapi-generator-cli generate -i asset/binance_rest_openapi.yaml -g python -o ./target_binance_rest --additional-properties=asyncio=true
```
### AsyncAPI model
```
asyncapi generate models <language> example_asyncapi.yaml -o output/example_<language>>_model
```
### AsyncAPI rust client (WS, tokio-tungstenite)
> WIP in [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template)

Missing now, I am hiring node.js dev to work on this one
### AsyncAPI python client (WS, asyncio-websockets)
> planned

## exchange integration status
below are the list of exchanges planned for integration. Please contact me if you want to integrate for orderbook exchange.
| Exchange                                                  | Custodial | REST (OpenAPI) | WS (AsyncAPI) |
| --------------------------------------------------------- | --------- | -------------- | ------------- |
| [hyperliquid](https://hyperliquid.gitbook.io)             | no        | done           | done          |
| [binance](https://binance-docs.github.io)                 | yes       | done           | done          |
| [coinbase](https://docs.cdp.coinbase.com)                 | yes       | done           | wip           |
| [hashkey](https://hashkeypro-apidoc.readme.io)            | yes       | done           | planned       |
| [bitwyre](https://docs.bitwyre.com)                       | no        | planned        | planned       |
| [dydx](https://docs.dydx.exchange)                        | no        | planned        | planned       |
| [injective](https://docs.injective.network)               | no        | planned        | planned       |
| [polkadex](https://docs.polkadex.trade)                   | no        | planned        | planned       |
| [zkex](https://docs.zkex.com)                             | no        | planned        | planned       |
| [gmx](https://gmx-docs.io)                                | no        | planned        | planned       |
| [bybit](https://bybit-exchange.github.io/docs/)           | yes       | planned        | planned       |
| [okx](https://www.okx.com/docs-v5/en)                     | yes       | planned        | planned       |
| [gateio](https://www.gate.io/docs/developers/apiv4)       | yes       | planned        | planned       |
| [kucoin](https://www.kucoin.com/docs)                     | yes       | planned        | planned       |
| [kraken](https://docs.kraken.com/api)                     | yes       | planned        | planned       |
| [htx](https://www.htx.com/en-us/opend/newApiPages/)       | yes       | planned        | planned       |
| [bitget](https://www.bitget.com/api-doc)                  | yes       | planned        | planned       |
| [bitflyer](https://lightning.bitflyer.com/docs)           | yes       | planned        | planned       |
| [coincheck](https://coincheck.com/documents/exchange/api) | yes       | planned        | planned       |
| [korbit](https://apidocs.korbit.co.kr)                    | yes       | planned        | planned       |

I currently have no plan of supporting [FIX protocol](https://www.fixtrading.org/what-is-fix) due to limited number of supported exchanges. But it is definitely an interesting one to try in the future.  

## TODO
- [ ] gather assets
  - [ ] gather initial assets
    - [x] gather 3 exchanges in OpenAPI
    - [ ] gather 3 exchanges in AsyncAPI
  - [ ] gather 10 exchanges for single exchange trading
  - [ ] gather 20 exchanges for cross exchange trading
- [ ] set up CI for codegen model
  - [ ] rust codegen
    - [x] REST (reqwest) client
    - [ ] WS (tokio-tungstenite) client template 
    - [ ] CI for release on [crates.io](https://crates.io)
  - [ ] python codegen
    - [x] REST client
    - [ ] WS (asyncio-websockets)
    - [ ] CI for release on [pip]()
- [x] set up [guilder](https://github.com/kanekoshoyu/guilder) trading library
  - [x] define trading traits
  - [ ] implement traits on top of the codegen model
  - [ ] package models with opinionated trait per language

## notes
- the `ag` command seems to be deprecated and cannot generate code properly
- you can install `asyncapi-preview` extension on vs code for preview
- comnunity AsyncAPI templates like `python-sanic-template` are not working properly 

## partnership
I keep this project opensource so that everyone can take part of it. If you have any OpenAPI / AsyncAPI document for a crypto exchange, you are more than welcome to add with a pull request, or I am willing to purchase as well.  
If you want to get an exchange integrated, I can help get that up for an one-off cost in one week, just enough to pay my freelancing partner to get it done.  
Please contact [Sho Kaneko](https://github.com/kanekoshoyu) for details.  

## recruitment
#### OpenAPI / AsyncAPI author
I am gathering API doc and it would be great if people can help me with it.  
#### javascript AsyncAPI template developer
> [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template)

I have set up a repo to develop AsyncAPI template for Rust WS in React. I am not a React expert, so I would love to have expert to accelarate development.  

## see also
- [guilder](https://github.com/kanekoshoyu/guilder) - Unopinionated Cross-Exchange Crypto Trading Library
- [asyncapi-rust-ws-template](https://github.com/kanekoshoyu/asyncapi-rust-ws-template) - AsyncAPI Template for Generating Rust WebSocket Client
- [kucoin-arbitrage](https://github.com/kanekoshoyu/kucoin_arbitrage) - KuCoin Cyclic Arbitrage, in Tokio Rust (legacy)