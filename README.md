# Hurricane Money
*No LLM was used in the writing of this code, other than for knowledge assistance*

## Overview

This project aims to be a basic implementation of [Tornado Cash](https://github.com/tornadocash/tornado-core) using [OpenVM](https://docs.openvm.dev/book/getting-started/introduction) for all ZK proof related parts.

There are two important packages here:

1. [Guest](./guest/)
2. [Host](./host/)

*Guest* covers the program that is proven, on withdrawal, i.e. in pseudo-code:

```rust
merkle_proof = generate_merkle_proof(merkle_tree, leaf)
process_merkle_proof(merkle_proof, hash(concat(k, r))) == root
```

where *k*, and *r*, are the *nullifier* and *randomness* respectively, *root* is the root hash of the Merkle tree. This pseudo-code is modified from this explainer: [How Tornado Cash Works (Line by Line for Devs) - RareSkills](https://web.archive.org/web/0/https://rareskills.io/post/how-does-tornado-cash-work).

*Host* uses the [OpenVM SDK](https://docs.openvm.dev/book/advanced-usage/sdk) to do everything else i.e. generate the whole Merkle tree structure and pass it into the [guest](./guest/src/main.rs) for proving. It also handles non-ZK user operations like deposit preparation (sampling `k`, `r`, and computing the commitment to send on-chain).

## Installation

[OpenVM - Install](https://docs.openvm.dev/book/getting-started/install)

## Building 

```rust
cargo openvm build --manifest-path guest/Cargo.toml
```

## Running

### Host

```rust
cargo run -p host
```

### Guest only

```rust
cargo openvm run --package guest --input guest/inputs.json --config guest/openvm.toml
```

## Generating Proof

```rust
cargo openvm keygen --config guest/openvm.toml
cargo openvm prove app --config guest/openvm.toml --bin guest --input guest/inputs.json
```

## Verifying Proof

```rust
cargo openvm verify app
```

