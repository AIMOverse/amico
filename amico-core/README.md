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
  - `agent_event.rs`: Defines the `AgentEvent` structure and `EventContent` enum
  - `instruction.rs`: Defines agent instructions like `Terminate`
- **`world/`**: World state management components including:
  - `manager.rs`: Core world management functionality
  - `delegate.rs`: Delegation patterns for world operations

## Key Concepts

- **Event-Driven Architecture**: The framework operates on an event-based model where `EventSource`s generate `AgentEvent`s that are processed by the agent.
- **Entity Component System**: Uses an ECS pattern for efficient state management and component organization.
- **Strategy Pattern**: Flexible event dispatching through the `Strategy` trait for customizable action selection logic.
- **Event Sources**: Components that generate events for the agent to process. They implement the `EventSource` trait and can be any source of information or stimuli for the agent, such as user input, timers, or external APIs.
- **Agent Events**: Represented by the `AgentEvent` struct, these are high-level events that carry information from event sources to the agent. They contain metadata (name, source), optional content (data or instructions), and can have an expiry time.
- **Agent Actions**: Unlike `AgentEvent`s which are external stimuli, Agent Actions are implemented as ECS events (using the `evenio` library's `Event` trait). These events represent internal state changes and behaviors within the agent's world model. The Strategy component translates external `AgentEvent`s into appropriate ECS events that modify the world state. 

**NOTE** The ECS Events will be used only in the crate's internal APIs and be hidden behind Actions in future versions.

## Documents

See Amico's website [https://amico.dev](https://amico.dev)

## License

This crate is released under the [MIT License](https://github.com/AIMOverse/amico/blob/main/LICENSE-MIT) **OR** the [Apache-2.0 License](https://github.com/AIMOverse/amico/blob/main/LICENSE-Apache-2.0)
