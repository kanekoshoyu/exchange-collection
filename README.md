# henlo: machine readable crypto exchange API doc in AsyncAPI YAML

there are way too many crypto exchanges and programming languages. we should try standardise the way we implement the exchange library.

I will add a bunch of AsyncAPI YAML here. In the near future I will set up a codegen that generates the rust exchange lib for python and crate for rust.

# Python
install async api code generator
`npm install -g @asyncapi/generator`
generate python code for kraken (installation fails)
`ag kraken-asyncapi.yml @asyncapi/python-sanic-template -o ./output_python`

`npm install -g @asyncapi/cli`
`asyncapi generate fromTemplate example_2.yml @asyncapi/python-paho-template -o output/example`