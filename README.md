# exchange_yaml: list of crypto exchange API doc in OpenAPI/AsyncAPI
[![doc](https://img.shields.io/badge/doc-rapidoc-blue)](https://example.com)
[![license](https://img.shields.io/github/license/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/blob/master/LICENSE)
[![ci](https://img.shields.io/github/actions/workflow/status/kanekoshoyu/kucoin_arbitrage/rust.yml)](https://github.com/kanekoshoyu/kucoin_arbitrage/actions)
[![issues](https://img.shields.io/github/issues/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/issues)
[![discussions](https://img.shields.io/github/discussions/kanekoshoyu/kucoin_arbitrage)](https://github.com/kanekoshoyu/kucoin_arbitrage/discussions)
[![discord](https://img.shields.io/discord/1153997271294283827)](https://discord.gg/q3j5MYdwnm)  

## problem with other cross-exchange libraries/frameworks
Most of the trading bots require data and execution at different exchanges. There are trading libraries that provides abstraction over multiple exchanges.
- [ccxt](https://github.com/ccxt/ccxt)
- [hummingbot](https://github.com/hummingbot/hummingbot)
- [barter_rs](https://github.com/barter-rs/barter-rs)

However, they often miss support in either exchanges or languages.
- high marginal effort: there are way too many crypto exchanges and programming languages.  Say there are N crypto exchanges, and L programming language, and the effort to convert those written API doc into exchange library is B, then the overall effort is N * L * B.
- poor versioning: crypto exchange usually have lots of API updates and takes lots of manual testing, and it comes with inconsistent API doc as well
- highly opinionated models: these cross-exchange libraries often goes with their bigger project/workspace, and unfortunately there is no thoughts in making them generic, so often you cannot use their models directly to fulfill your actual needs.

## solution
I want to streamline the process of integrating multi-exchange API with a new approach to this problem. The idea is to gather a list of OpenAPI/AsyncAPI from both official and unofficial sources under a few guidelines, and try to generate a bunch of generic code ready for your use.
1. gather both OpenAPI and AsyncAPI YAML
(effort: N * S, S is the smaller effort of gathering / testing YAML from written API)
   - OpenAPI: define REST API
   - AsyncAPI: define event-driven API (websocket)
1. Set up codegen CI for generating models for L languages
(effort: constant, codegen is available and just require some initial set up)
1. Implement trading abstraction or interface per generated model.
(effort: N * L * S)

## current status
To kick off, I will gather a bunch of AsyncAPI YAML here. In the near future I will set up a codegen that generates the rust exchange lib for python and crate for rust.
| Exchange    | OpenAPI YAML | AsyncAPI YAML |
| ----------- | ------------ | ------------- |
| Binance     | Done         | WIP           |
| Hyperliquid | Done         | planned       |
| Coinbase    | Done         | planned       |
| HashKey     | WIP          | planned       |
| KuCoin      | planned      | planned       |
| ByBit       | planned      | planned       |

## guidelines
1. OpenAPI: YAML, v3+
2. AsyncAPI: YAML, v2 (codegen does not work well with v3 apparently)
3. official codegen support: rust, python
4. unofficial support: typescript, csharp, golang, java, javascript, dart,kotlin, php, cplusplus, scala
5. place YAML in follow the naming convension of `{exchange}_{asyncapi/openapi}.yaml`
6. codegen scripting in python

## codegen commands
below are the commands
### OpenAPI
#### initial set up
install OpenAPI CLI
```
npm install -g @openapitools/openapi-generator-cli
```
#### rust model
```
openapi-generator-cli generate -i path/to/your/openapi.yaml -g <language> -o path/to/output/directory
```
#### rust model
```
openapi-generator-cli generate -i path/to/your/openapi.yaml -g <language> -o path/to/output/directory
```


### AsyncAPI
#### initial set up

install AsyncAPI CLI
```
npm install -g @asyncapi/generator
```
#### rust model
```
asyncapi generate models rust example_asyncapi.yml -o output/example_rust_model
```
#### python model
```
asyncapi generate models python example_asyncapi.yml -o output/example_python_model
```
#### python paho
```
asyncapi generate fromTemplate example_asyncapi.yml @asyncapi/python-paho-template -o output/example_python_paho
```

## todo
- [x] gather 3+ exchanges in both OpenAPI
- [ ] gather 3+ exchanges in both AsyncAPI
- [ ] verify if models can be generated on python/rust, and publish it in
- [ ] verify if traits could be implemented on top of the generated model
- [ ] package model with its version

## notes
- the `ag` command seems to be deprecated and cannot generate code properly
- you can install `asyncapi-preview` extension on vs code for preview
- AsyncAPI `python-sanic-template` was failing