technical_indicators [![Build Status](https://travis-ci.org/dashed/technical_indicators.svg)](https://travis-ci.org/dashed/technical_indicators) [![crate version](https://img.shields.io/crates/v/technical_indicators.svg?style=flat)](https://crates.io/crates/technical_indicators) [![docs.rs](https://docs.rs/technical_indicators/badge.svg)](https://docs.rs/technical_indicators)
====================

> Library of technical indicators in Rust.

## What?

A technical indicator is a tool that attempts to forecast a technical analysis feature (e.g. support / resistance) based on given data such as historical price and volume.

Typically, a trader or an investor would use technical indicators against an asset (e.g. stocks), and then perform technical analysis to formulate a trading (or investing) decision.

See: https://en.wikipedia.org/wiki/Technical_indicator

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
technical_indicators = "0.5.0"
```

Next, add this to your crate:

```rust
extern crate technical_indicators;
```

## Usage

*TBA.*

Indicators
==========

Implemented indicators.

## Simple Moving Average

Read more: https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average

## Exponential Moving Average

Read more: https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average

## Ichimoku Kinkō Hyō

Read more: https://en.wikipedia.org/wiki/Ichimoku_Kink%C5%8D_Hy%C5%8D

Chores
======

- `./pretty.sh`: Run [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt)
- `cargo test`: Run tests.
- `cargo check`: Quick compile check.

Credits
=======

Thanks to [Cryptowatch API](https://cryptowat.ch/docs/api#ohlc) for the bitcoin trading data of the GDAX (Coinbase) exchange.

URL used to access the trading data: `https://api.cryptowat.ch/markets/gdax/btcusd/ohlc?periods=14400` (4 hour candles)

License
=======

MIT.
