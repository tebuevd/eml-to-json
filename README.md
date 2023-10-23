# eml-to-json

A CLI tool to read an .eml file and print out a JSON representation to stdout.

## Installation

```
git clone https://github.com/tebuevd/eml-to-json
cd eml-to-json
cargo install --path .
```

## How to use

```
eml-to-json ~/Documents/email.eml
```

or with jq for pretty-printing

```
eml-to-json ~/Documents/email.eml | jq
```

### License

MIT
