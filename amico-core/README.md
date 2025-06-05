# Amico Core

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the core engine for the Amico AI Agent Framework, including the following features:

1. Event-driven agent architecture with a robust event handling system
2. Entity Component System (ECS) integration for efficient state management
3. Flexible event dispatching and action selection mechanisms
4. World state management and delegation for agent operations

## Directory Structure

The crate is organized as follows:

### Source Code (`src/`)

- **`agent.rs`**: Defines the core `Agent` struct that manages the event loop and event sources
- **`ecs.rs`**: Provides Entity Component System (ECS) integration 
- **`errors/`**: Defines error types and handling for the framework
- **`traits/`**: Core interfaces including:
  - `Strategy`: Action selection strategies
  - `EventSource`: Event generation interfaces
  - `System`: ECS system interfaces
  - `handlers`: Event handling mechanisms
- **`types/`**: Contains concrete type definitions for events, instructions, and data structures
- **`world/`**: World state management components including:
  - `manager.rs`: Core world management functionality
  - `delegate.rs`: Delegation patterns for world operations

## Key Concepts

- **Event-Driven Architecture**: The framework operates on an event-based model where `EventSource`s generate `AgentEvent`s that are processed by the agent.
- **Entity Component System**: Uses an ECS pattern for efficient state management and component organization.
- **Strategy Pattern**: Flexible event dispatching through the `Strategy` trait for customizable action selection logic.
- **World Management**: The `WorldManager` provides structured access to the agent's world state.

## Documents

See Amico's website [https://amico.dev](https://amico.dev)

## License

This crate is released under the [MIT License](https://github.com/AIMOverse/amico/blob/main/LICENSE-MIT) **OR** the [Apache-2.0 License](https://github.com/AIMOverse/amico/blob/main/LICENSE-Apache-2.0)
