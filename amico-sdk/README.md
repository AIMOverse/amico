# Amico SDK

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the Software Development Kit (SDK) for the Amico AI Agent Framework, including the following features:

1. AI service abstractions and interfaces;
2. Task execution workflow and context management;
3. Environment interaction through sensors and effectors;
4. Core agent models and action handling.

## Directory Structure

The crate is organized as follows:

### Core Components

- **`ai/`**: AI-related abstractions and implementations.
  - **`completion/`**: Interfaces for text completion services.
  - **`embedding/`**: Embedding generation and management.
  - **`message/`**: Message structures for AI communication.
  - **`provider/`**: Provider interfaces for AI services.
  - **`service/`**: Service abstractions for AI capabilities.
  - **`tool/`**: Tool definitions for AI agent interactions.

- **`core/`**: Core functionality for agent operations.
  - **`action_map/`**: Action mapping and selection.
  - **`ai_action/`**: AI action definitions and handling.
  - **`model/`**: Model definitions and abstractions.

### Interaction Components

- **`environment.rs`**: Environment interaction through sensors and effectors.
- **`interaction/`**: User and agent interaction interfaces.
- **`resource.rs`**: Resource management for agent operations.
- **`task.rs`**: Task definition and execution workflow.

> **Note**: The Amico SDK provides a high-level abstraction layer for building AI agents, allowing developers to focus on agent logic rather than implementation details of AI services and environment interactions.

## Documents

- [SDK Module reference](https://www.amico.dev/docs/modules/amico-sdk)
- [Agent development guide](https://www.amico.dev/docs/getting-started/sdk-guide)

## License

This crate is released under the [**MIT License**](https://github.com/AIMOverse/amico/blob/main/LICENSE)
