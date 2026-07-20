# rust-tinkoff-invest-types

[![Rust](https://github.com/WatchDG/rust-tinkoff-invest-types/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/WatchDG/rust-tinkoff-invest-types/actions/workflows/rust.yml)

Protobuf/gRPC types for the [T-Bank (Tinkoff) Invest API](https://developer.tbank.ru/invest/intro/intro).

[CONTRACTS](https://opensource.tbank.ru/invest/invest-contracts) | [DOCS](https://developer.tbank.ru/invest/intro/intro) | [docs.rs](https://docs.rs/tinkoff-invest-types)

## Install

```toml
[dependencies]
tinkoff-invest-types = "2"
```

```rust
use tinkoff_invest_types::PortfolioRequest;

fn main() {
    let _ = PortfolioRequest::default();
}
```

## Requirements

- **protoc** — [Protocol Buffers compiler](https://grpc.io/docs/protoc-installation/) must be available on `PATH` at build time.
- **Git submodule** — when building from this repository (not from crates.io), initialize contracts first:

```bash
git submodule update --init --recursive
```
