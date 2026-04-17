# Hurricane Money
*No LLM was used in the writing of this code, other than for knowledge assistance*

## Overview

This project aims to be a basic implementation of [Tornado Cash](https://github.com/tornadocash/tornado-core) using [OpenVM](https://docs.openvm.dev/book/getting-started/introduction) for all ZK proof related parts.

There are two important packages here:

1. [Program](./program/)
2. [Prover](./prover/)

*Program* covers the program that is proven i.e. in pseudo-code:

```rust
merkle_proof = generate_merkle_proof(merkle_tree, leaf)
process_merkle_proof(merkle_proof, hash(concat(k, r))) == root
```

where *k*, and *r*, are the *nullifier* and *randomness* respectively, *root* is the root hash of the Merkle tree. This pseudo-code is modified from this explainer: [How Tornado Cash Works (Line by Line for Devs) - RareSkills](https://web.archive.org/web/0/https://rareskills.io/post/how-does-tornado-cash-work).

*Prover* uses the [OpenVM SDK](https://docs.openvm.dev/book/advanced-usage/sdk) to do everything else i.e. generate the whole Merkle tree structure and pass it into the [program](./program/src/main.rs) for proving.

## Installation

[OpenVM - Install](https://docs.openvm.dev/book/getting-started/install)

## Building 

```rust
cargo openvm build --manifest-path program/Cargo.toml
```

## Running

```rust
cargo openvm run --package program --input program/inputs.json --config program/openvm.toml
```

## Generating Proof

```rust
cargo openvm keygen --config program/openvm.toml
cargo openvm prove app --config program/openvm.toml --bin program --input program/inputs.json
```

## Verifying Proof

```rust
cargo openvm verify app
```

