# inditech

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/stjepangolemac/inditech/main/LICENSE)

A set of technical indicators that can be used for time series analysis.
There's only a few of them for now but new ones might be added soon.

## Usage

```rust
use inditech::{Frame, Identity, EMA, ROC, RSI, SMA};

fn main() {
    let mut frame = Frame::new();
    frame.add(Box::new(Identity::new()));
    frame.add(Box::new(SMA::new(8)));
    frame.add(Box::new(EMA::new(8)));
    frame.add(Box::new(RSI::new(8)));
    frame.add(Box::new(ROC::new(8)));

    let data: Vec<f32> = generate_data();
    data.iter().for_each(|item| frame.push(item));

    dbg!(frame.last());
    // Vec[<data>, <sma8>, <ema8>, <rsi8>, <roc8>]
}
```
