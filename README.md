# exchange-collection
> Crypto Exchange OpenAPI and Generated Models

[![doc](https://img.shields.io/badge/doc-rapidoc-blue)](https://repoch.co/exchange_yaml)
[![license](https://img.shields.io/github/license/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/blob/master/LICENSE)
[![ci](https://img.shields.io/github/actions/workflow/status/kanekoshoyu/kucoin_arbitrage/rust.yml)](https://github.com/kanekoshoyu/kucoin_arbitrage/actions)
[![issues](https://img.shields.io/github/issues/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/issues)
[![discussions](https://img.shields.io/github/discussions/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/discussions)
[![discord](https://img.shields.io/discord/1153997271294283827)](https://discord.gg/q3j5MYdwnm)  

## problem with other cross-exchange libraries/frameworks
Most of the trading bots require data and execution at different exchanges. There are trading libraries that provides abstraction over multiple exchanges.

| library / framework                                    | multi-exchange | primary language | wrapper language    |
| ------------------------------------------------------ | -------------- | ---------------- | ------------------- |
| [ccxt](https://github.com/ccxt/ccxt)                   | yes            | js               | python, php, csharp |
| [hummingbot](https://github.com/hummingbot/hummingbot) | yes            | python, cpp      | n/a                 |
| [openlimits](https://github.com/nash-io/openlimits)    | yes            | rust             | python, js, go      |
| [barter_rs](https://github.com/barter-rs/barter-rs)    | yes            | rust             | n/a                 |

They often miss support in either exchanges or languages:
1. high marginal effort: there are way too many crypto exchanges and programming languages.  Say there are N crypto exchanges, and L programming language, and the effort to convert those written API doc into exchange library is B, then the overall effort is N * L * B.
2. inconsistecy along with updates: crypto exchange have freqent API updates and often leads to inconsistency with API doc.
3. opinionated models: these cross-exchange libraries often goes with their bigger project/workspace, and unfortunately there is no thoughts in making them generic, so often you cannot use their models directly to fulfill your actual needs. Also they use wrappers, so natually they are under-optimized.

## solution
I propose a more streamlined integration of exchange API with a new approach to this problem. The idea is to gather a list of OpenAPI/AsyncAPI from both official and unofficial sources under a few guidelines, and try to generate a bunch of generic code ready for your use.
1. gather both OpenAPI and AsyncAPI YAML
(N * S, S: small effort of gathering API doc)
   - OpenAPI: define REST API
   - AsyncAPI: define event-driven API (websocket)
1. Set up codegen CI for generating models for L languages
(effort: constant, codegen is available and just require some initial set up)
1. Implement minimally opinionated trading interface per generated model.
(effort: N * L * S)

## structure
| location                     | feature                                                                         |
| ---------------------------- | ------------------------------------------------------------------------------- |
| [asset](./asset/)            | OpenAPI and AsyncAPI YAML                                                       |
| script                       | codegen script in python, since CI installing rust every run is :shit:          |
| [target](./target/README.md) | generated code in python and rust                                               |
| [index.html](./index.html)   | Rapidoc OpenAPI YAML viewer, hosted [here](https://www.repoch.co/exchange_yaml) |

## guidelines
1. OpenAPI: YAML, v3.0.1 (convert swagger to OpenAPI [here](https://editor.swagger.io/))
2. AsyncAPI: YAML, v2 (codegen does not work well with v3 apparently)
3. place YAML in follow the naming convention of `{exchange}_{rest/ws}_{openapi/asyncapi}.yaml`
4. codegen scripting in python (for easier GitHub CI)
5. official codegen support: rust, python
6. unofficial support: typescript, csharp, golang, java, dart, kotlin, php, cplusplus, scala

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
### OpenAPI
#### model
```
openapi-generator-cli generate -i example_openapi.yaml -g <language> -o output/example_rust_model
```


### AsyncAPI
#### model
```
asyncapi generate models <language> example_asyncapi.yml -o output/example_<language>>_model
```
#### python paho client (opinionated imo, for reference only)
```
asyncapi generate fromTemplate example_asyncapi.yml @asyncapi/python-paho-template -o output/example_python_paho
```

## current status
To kick off, I will gather a bunch of AsyncAPI YAML here. In the near future I will set up a codegen that generates the rust exchange lib for python and crate for rust.
| Exchange    | Custody | REST (OpenAPI) | WS (AsyncAPI) |
| ----------- | ------- | -------------- | ------------- |
| hyperliquid | no      | done           | planned       |
| binance     | yes     | done           | WIP           |
| coinbase    | yes     | done           | planned       |
| hashkey     | yes     | done           | planned       |
| bitwyre     | no      | planned        | planned       |
| dydx        | no      | planned        | planned       |
| gmx         | no      | planned        | planned       |
| bybit       | yes     | planned        | planned       |
| kucoin      | yes     | planned        | planned       |
| okx         | yes     | planned        | planned       |

I currently have no plan of supporting [FIX protocol](https://www.fixtrading.org/what-is-fix/) due to limited number of supported exchanges. But it is definitely an interesting one to try in the future.


## TODO
- [ ] gather assets
  - [x] gather 3 exchanges in OpenAPI
  - [ ] gather 3 exchanges in AsyncAPI
- [x] set up CI for codegen model
  - [x] verify if the codegen models can be generated on python/rust
  - [x] package codegen model another repo per language
  - [ ] make it available in pip and crate.io
- [x] set up trading library
  - [x] define trading traits
  - [ ] implement traits on top of the codegen model
  - [ ] package models with opinionated trait per language

## notes
- the `ag` command seems to be deprecated and cannot generate code properly
- you can install `asyncapi-preview` extension on vs code for preview
- AsyncAPI `python-sanic-template` was failing

## partnership
I keep this project opensource so that everyone can take part of it. If you have any OpenAPI / AsyncAPI document for a crypto exchange, you are more than welcome to add with a pull request, or I am willing to purchase as well.  
If you want to get an exchange integrated, I can help get that up for an one-off cost in one week, just enough to pay my freelancing partner to get it done.  
Please contact [Sho Kaneko](https://github.com/kanekoshoyu) for details.

## see also
- [guilder](https://github.com/kanekoshoyu/guilder) - Unopinionated Cross-Exchange Crypto Trading Library
- [kucoin-arbitrage](https://github.com/kanekoshoyu/kucoin_arbitrage) - KuCoin Cyclic Arbitrage, in Tokio Rust (legacy)