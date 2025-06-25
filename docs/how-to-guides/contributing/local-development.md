# Local development

## Prerequisites

- Install [Rust](https://rustup.rs/) (1.70 or later)
- Install [Nix](https://nixos.org/download) (optional, for development shell)

## Development Shell (Optional)

Open the development shell:

```sh
nix develop
```

It will open a shell with all the necessary tools for local development.

## Run

To run the project with sample data:

```sh
# Using the simple example
cargo run -- examples/podinfo/values.toml

# Using the full example
cargo run -- examples/full-example.toml
```

Or with make:

```sh
make dev
```

## Build

To build the project:

```sh
cargo build --release
```

Or with make:

```sh
make
```

## Test

To run the tests:

```sh
cargo test
```

Or with make:

```sh
make test
```

## Examples

The project includes two example configurations:

- `examples/podinfo/values.toml`: Simple deployment with service
- `examples/full-example.toml`: Complete example with all supported resources
