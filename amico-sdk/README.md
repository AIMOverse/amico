# Amico SDK

This crate is a part of the [**Amico** project](https://github.com/AIMOverse/amico), a powerful and flexible AI agent framework.

## What does this crate do

This crate provides the SDK for Amico AI Agent Framework's core features, including:

1. Completion model and session management;
2. Core AI modules including tools and messages;
3. Agent-to-Agent (A2A) communication capabilities;
4. Agent on Environment (AoE) execution framework;
5. Runtime platform abstractions;
6. Global resource management.

## Directory Structure

- **`ai/`**: AI-related abstractions and implementations.
  - **`completion/`**: Interfaces for text completion models and sessions.
    - **`model.rs`**: Completion model abstractions.
    - **`session.rs`**: Session management for stateful completions.
    - **`error.rs`**: Error handling for completion operations.
  - **`mcp/`**: **Model Context Protocol** implementation.
    - **`client.rs`**: MCP client for model interaction.
    - **`tool.rs`**: Tool implementations for MCP.
  - **`message.rs`**: Message structures for AI communication.
  - **`tool.rs`**: Tool definitions for AI agent interactions.

- **`a2a/`**: Agent-to-Agent communication framework.
  - **`network.rs`**: Networking components for agent communication.

- **`runtime/`**: Runtime platform abstractions.
  - **`storage.rs`**: Storage interfaces and implementations.

- **`environment.rs`**: Environment interaction through sensors and effectors.
- **`resource.rs`**: Global resource management for agent operations.
- **`aoe.rs`**: Agent on Environment (AoE) execution components.

## Key Concepts

- **Completion Model and Session**: Abstractions for working with AI completion models and managing stateful completion sessions.
- **Tools and Messages**: Core components for AI interaction, with tools defining actions agents can perform and messages structuring communication.
- **Agent-to-Agent (A2A)**: Framework enabling agents to communicate and collaborate with each other.
- **Agent on Environment (AoE)**: System for agents to perceive and act upon their environment.
- **Runtime Platform Abstraction**: Cross-platform support allowing agents to operate in various environments.
- **Global Resources**: Centralized resource management for shared access across the agent ecosystem.

## Documentation

See Amico's website [https://amico.dev](https://amico.dev)

