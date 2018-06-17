# Pixie Rust

A Recommender based on [Pinterest's Pixie Recommender][pixie].

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
an [anime recommendations database from Kaggle][anime-dataset].

[pixie]: https://dl.acm.org/citation.cfm?id=3186183
[anime-dataset]: https://www.kaggle.com/CooperUnion/anime-recommendations-database
