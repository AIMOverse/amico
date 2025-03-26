# Amico Mods

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the plugin system and implementations for the Amico AI Agent Framework, including the following features:

1. Plugin management and interfaces;
2. Standard AI capabilities and core functionality plugins;
3. Web3 integration for blockchain platforms like Ethereum and Solana.

## Directory Structure

The crate is organized as follows:

### Core Components

- **`interface/`**: Defines the core traits and interfaces for the plugin system.
  - **`plugin.rs`**: Provides the `Plugin` trait and related structures.
  - **`provider.rs`**: Contains provider interfaces for plugin functionality.

- **`plugin_manager/`**: Implements the plugin loading and management system.

> **⚠️ IMPORTANT**: The plugin interfaces and plugin manager are currently under active development and subject to change. API stability is not guaranteed. Developers are encouraged to provide feedback on the current design to help shape the final stable API.

### Plugin Implementations

- **`std/`**: Standard plugins for general AI agent capabilities.
  - **`ai/`**: AI-related plugins for language models and other AI services.
  - **`core/`**: Core functionality plugins for basic agent operations.

- **`web3/`**: Blockchain and Web3 integration plugins.
  - **`ethereum/`**: Ethereum blockchain integration.
  - **`solana/`**: Solana blockchain integration.
  - **`wallet.rs`**: Cryptocurrency wallet functionality.

## Features

- **`std`**: Enables standard plugins (enabled by default).
- **`web3`**: Enables basic Web3 functionality.
- **`web3-solana`**: Enables Solana blockchain support.
- **`web3-ethereum`**: Enables Ethereum blockchain support.
- **`full`**: Enables all available plugins and features.

## Documents

- [Modules (plugins) documentation](https://www.amico.dev/docs/modules/amico-mods)
- [Modules (plugins) development guide](https://www.amico.dev/docs/getting-started/modules-dev)

## License

This crate is released under the [**MIT License**](https://github.com/AIMOverse/amico/blob/main/LICENSE)
