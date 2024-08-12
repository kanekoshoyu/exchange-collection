# exchange_yaml: machine readable crypto exchange API doc in OpenAPI/AsyncAPI YAML

## problem
Most of the trading bots require data and execution at different exchanges. There are trading libraries like [CCXT](https://github.com/ccxt/ccxt) and [barter](https://github.com/barter-rs/barter-rs) that provides abstraction over multiple exchanges. However, they often miss support in either exchanges or languages.
- high effort
There are way too many crypto exchanges and programming languages.  Say there are N crypto exchanges, and L programming language, and the effort to convert those written API doc into exchange library is B, then the overall effort is N * L * B. 
- versioning
crypto exchange usually have lots of API updates and takes lots of manual testing, and it comes with inconsistent API doc as well
- too much of its flavor
these cross-exchange libraries often goes with their bigger project/workspace, making it hard to fulfill your needs.


## solution
Ultimately I want to streamline this process of integrating multi-exchange API. I propose a new approach to this problem with the help of crowdsourcing and opensourcing, and try to generate a bunch of generic code ready for your use.
1. gather both OpenAPI YAML and AsyncAPI YAML
(effort: N * S, S is the smaller effort of gathering / testing YAML from written API)
   - OpenAPI: define REST API
   - AsyncAPI: define event-driven API (websocket)
1. Set up codegen CI for generating models for L languages
(effort: constant, codegen is available and just require some initial set up)
1. Implement trading abstraction or interface per generated model.
(effort: N * L * S)


To kick off, I will gather a bunch of AsyncAPI YAML here. In the near future I will set up a codegen that generates the rust exchange lib for python and crate for rust.


## current status
| Exchange    | OpenAPI YAML | AsyncAPI YAML |
| ----------- | ------------ | ------------- |
| Binance     | Done         | WIP           |
| Hyperliquid | Done         | planned       |
| HashKey     | planned      | planned       |
| KuCoin      | planned      | planned       |
| ByBit       | planned      | planned       |



## codegen commands
install async api code generator
```
npm install -g @asyncapi/generator
```
### rust model
```
asyncapi generate models rust example_2.yml -o output/example_rust_model
```
### python model
```
asyncapi generate models python example_2.yml -o output/example_python_model
```
### python paho
```
asyncapi generate fromTemplate example_2.yml @asyncapi/python-paho-template -o output/example_python_paho
```
### python sanic (failing)
```
asyncapi generate fromTemplate example_2.yml @asyncapi/python-sanic-template -o output/example_python_sanic
```

## note
- apparently the these templates only work on AsyncAPI v2, not v3 (kraken_3.yml does not work)
- model gen supports: typescript, csharp, golang, java, javascript, dart, python, rust, kotlin, php, cplusplus, scala
- the `ag` command seems to be deprecated and cannot generate code properly
- you can install `asyncapi-preview` extension on vs code for preview
- follow the naming convension of `{exchange}_{asyncapi/openapi}.yaml` for easier browsing and scripting.

## todo
- gather 3+ exchanges
- verify if the binance models can be generated on python/rust
- verify if traits could be implemented on top of the generated model
- package model with its version
