technical_indicators
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
technical_indicators = "0.1.0"
```

Next, add this to your crate:

```rust
extern crate technical_indicators;
```

## Usage

*TBA.*

TODO
====

- [ ] ichimoku
- [ ] SMA
- [ ] EMA
- [ ] https://www.incrediblecharts.com/indicators/chandelier_exits.php

Indicators
==========

*TBA.*

Chores
======

- `cargo fmt`: Run [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt)

Credits
=======

Thanks to [Cryptowatch API](https://cryptowat.ch/docs/api#ohlc) for the bitcoin trading data of the GDAX (Coinbase) exchange.

URL used to access the trading data: `https://api.cryptowat.ch/markets/gdax/btcusd/ohlc?periods=14400` (4 hour candles)

License
=======

MIT.
