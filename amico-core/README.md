# Amico Core

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the core engine for the Amico AI Agent Framework, including the following features:

1. Engine layer interfaces and workflows;
2. Action selection and event generation mechanisms;
3. Core controller functionality for agent behavior.

## Directory Structure

The crate is organized as follows:

### Core Components

- **`core-macros/`**: A sub-crate providing procedural macros for `amico-core`, enhancing code generation and reducing boilerplate.

### Source Code (`src/`)

- **`controller/`**: Implements the agent workflow and execution logic.
- **`entities/`**: Contains concrete type definitions for events, actions, and state management.
- **`errors/`**: Defines error types and handling for actions, event pools, and action selectors.
- **`traits/`**: Provides core interfaces for actions, events, and action selection mechanisms.

### Deprecated Components

- **`config/`**: ⚠️ Deprecated module for configuration handling. Configuration features have been migrated to the runtime crate and this module will be removed in a future release.

## Documents

- [Framework architecture overview](https://www.amico.dev/docs/architecture-overview)
- [Core Module reference](https://www.amico.dev/docs/modules/amico-core)

## License

This crate is released under the [**MIT License**](https://github.com/AIMOverse/amico/blob/main/LICENSE)
