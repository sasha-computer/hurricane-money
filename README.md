# Hurricane Money
*No LLM was used in the writing of this code, other than for knowledge assistance*

This project aims to be a basic implementation of [Tornado Cash](https://github.com/tornadocash/tornado-core) using [OpenVM](https://docs.openvm.dev/book/getting-started/introduction) for all ZK proof related parts.

## Installation

[OpenVM - Install](https://docs.openvm.dev/book/getting-started/install)

## Building 

```rust
cargo openvm build
```

## Executing only

```rust
cargo openvm run --input inputs.json
```

## Generating Proof

```rust
cargo openvm keygen
cargo openvm prove app --input inputs.json
```

## Verifying Proof

```rust
cargo openvm verify app
```

