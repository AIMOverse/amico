# Amico AI Agent

Amico is an AI Agent Framework designed for DePin devices.

## Quick Start

### Install & Run Amico Immediately

```bash
cargo install amico

# This will run Amico using the default config.
amico
```

## Development Guide

Amico is built with Rust and is hosted on [GitHub](https://github.com/AIMOverse/amico).

First, clone the repository:

```bash
git clone https://github.com/AIMOverse/amico.git
cd amico
```

Then, run `cargo build` to build the library and the agent binary.

```bash
cargo build
```

### Plugin Development

Amico plugins are written in Rust and can be found in the `plugins` directory.

To add a new plugin:

```bash
cd plugins
cargo new --lib --vcs none amico-plugin-<name>
```

Then, edit the `Cargo.toml` file and add `description"` field under `[package]`.
