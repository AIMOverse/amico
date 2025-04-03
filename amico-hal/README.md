# Amico HAL

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the Hardware Abstraction Layer (HAL) for the Amico AI Agent Framework, including the following features:

1. Cross-platform audio interfaces for playback and recording;
2. OS-specific implementations for native platforms;
3. WebAssembly (WASM) support for web-based applications.

## Directory Structure

The crate is organized as follows:

### Core Components

- **`interface/`**: Defines the core traits and interfaces that abstract hardware functionality.
  - **`audio.rs`**: Provides traits for audio playback and recording capabilities.

### Platform Implementations

- **`os/`**: Contains native operating system implementations.
  - **`common/`**: Shared functionality across different operating systems.
  - **`linux/`**: Linux-specific implementations.

- **`wasm/`**: WebAssembly implementation for web browsers.

## Features

- **`os-common`**: Enables the common OS-specific audio functionality using cpal, hound, lame, and rodio libraries (enabled by default).

## Documents

- [HAL Module reference](https://www.amico.dev/docs/modules/amico-hal)

## License

This crate is released under the [**MIT License**](https://github.com/AIMOverse/amico/blob/main/LICENSE)
