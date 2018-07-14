# Pixie Rust

A Rust implementation of a recommender system based on the [Pinterest's Pixie recommender][pixie].

[Online Demo][demo]

**Disclaimer:** This is a toy project and should probably not be used in production.

## Features

### Implemented

- Biased random walk (with configurable weight functions)
- Multiple query pins with weights

### Not Implemented

- Early stopping
- Graph prunning
  - The prunning strategy is application specific.
- EdgeVec Graph
  - This data structure limits the possible weight functions.

## Examples

There is a simple recommender example in the `examples` folder based on
an [anime recommendations database from Kaggle][anime-dataset]. There is
an [online demo][demo] based on this example.

[pixie]: https://dl.acm.org/citation.cfm?id=3186183
[demo]: https://jd557.github.io/pixie-wasm/
[anime-dataset]: https://www.kaggle.com/CooperUnion/anime-recommendations-database
