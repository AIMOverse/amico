# Amico V2 AI Agent Framework

**⚠️ MAJOR VERSION UPDATE: This is a complete rewrite of Amico (V2)**

Amico V2 is a platform-agnostic **runtime** for AI agents built in Rust. As a framework, it provides a platform for developers to develop their business logic - just like web frameworks like Axum or Rocket.

## 📚 Documentation

For detailed architecture design, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## Links

[![Website](https://img.shields.io/badge/Website-aimo.network-blue?style=for-the-badge&logo=globe)](https://aimo.network)
[![Docs](https://img.shields.io/badge/Docs-amico.dev-green?style=for-the-badge&logo=book)](https://www.amico.dev)
[![Paper](https://img.shields.io/badge/Paper-arXiv-red?style=for-the-badge&logo=arxiv)](http://arxiv.org/abs/2507.14513)
[![Discord](https://img.shields.io/badge/Discord-Join%20Community-7289da?style=for-the-badge&logo=discord)](https://discord.gg/MkeG9Zwuaw)

## 🚀 What's New in V2

Amico V2 is a **complete redesign** from the ground up, incorporating modern AI agent framework patterns and best practices:

- **Platform-Agnostic Runtime**: Run on any platform - OS, browsers, mobile, embedded devices
- **Model Abstraction Layer**: Provider-agnostic model interface inspired by Vercel's AI SDK
- **Workflow Runtime**: Support for both long-lived and short-lived runtimes
- **Zero-Cost Abstractions**: Uses traits and generics instead of dynamic dispatch
- **Type-Safe**: Extensive compile-time verification
- **Event-Driven Architecture**: Framework-like event handler interface

## Architecture Overview

Amico V2 consists of four layers:

```
┌─────────────────────────────────────────┐
│     Application / Event Handlers        │  ← Your business logic
├─────────────────────────────────────────┤
│     Workflows Layer (Presets)           │  ← Tool loop agents, ReAct, etc.
├─────────────────────────────────────────┤
│     Runtime Layer                        │  ← Workflow execution
├─────────────────────────────────────────┤
│     Models Layer                         │  ← Model abstractions
├─────────────────────────────────────────┤
│     System Layer                         │  ← Tools, side-effects, I/O
└─────────────────────────────────────────┘
```

### Core Concepts

1. **Models Layer** (`amico-models`): Abstracts AI models by capability (language, image, video, speech, embedding)
2. **System Layer** (`amico-system`): Defines how agents interact with the world through tools and side-effects
3. **Runtime Layer** (`amico-runtime`): Executes workflows on different runtime types (long-lived or short-lived)
4. **Workflows Layer** (`amico-workflows`): Preset workflow patterns (tool loops, ReAct, chain-of-thought, etc.)

## Design Principles

- **Traits + Generics over Boxing**: Always use traits and generics for abstractions, avoid dynamic dispatch
- **Compile-time Safety**: Extensive use of types to catch errors early
- **Zero-cost Abstractions**: No runtime overhead from abstraction layers
- **Lifetime-aware**: Explicit lifetime management instead of `Box` or `Arc`
- **Async-native**: All async operations use `impl Future`, not `Pin<Box<dyn Future>>`

## Modules

1. **`amico`**: The main entry crate that ties everything together
2. **`amico-models`**: Model abstractions categorized by capability
3. **`amico-system`**: System layer for tools and side-effects
4. **`amico-runtime`**: Runtime layer for executing workflows
5. **`amico-workflows`**: Preset workflow patterns

## Example Usage

```rust
use amico::{
    EventHandler, EventRouter,
    runtime::Runtime,
    workflows::ToolLoopAgent,
};

// Define your event handler
struct MyAgentHandler {
    agent: ToolLoopAgent<MyModel, MyTools, MyContext>,
}

impl EventHandler<MessageEvent> for MyAgentHandler {
    type Context = AgentContext;
    type Response = MessageResponse;
    type Error = HandlerError;
    
    async fn handle(&self, event: MessageEvent, context: &Self::Context)
        -> Result<Self::Response, Self::Error>
    {
        let response = self.agent
            .execute(context, event.content)
            .await?;
        Ok(MessageResponse::from(response))
    }
}

// Create runtime and register handlers
async fn main() {
    let runtime = create_runtime();
    let mut router = EventRouter::new();
    
    // Register event handlers
    router.register("message", MyAgentHandler::new());
    router.register("timer", TimerHandler::new());
    
    // Start runtime
    runtime.start().await.unwrap();
}
```

## Migration from V1

**V1 is completely deprecated and not compatible with V2.**

V2 is a total rewrite that keeps some good concepts from V1 (event-based architecture, platform-agnostic design) while:

- Removing all dynamic dispatch in favor of static generics
- Clarifying separation between models, runtime, system, and workflows
- Providing clearer abstractions inspired by modern frameworks like Vercel's AI SDK
- Focusing on compile-time safety and zero-cost abstractions

## 🚧 Development Status

**V2 is currently in design phase.** The architecture and traits are defined, but implementations are placeholders. We're building the conceptual framework first before implementing functionality.

## License

Amico is released under the [MIT License](https://github.com/AIMOverse/amico/blob/main/LICENSE-MIT) **OR** the [Apache-2.0 License](https://github.com/AIMOverse/amico/blob/main/LICENSE-Apache-2.0).

### Images

All images under `images/` are licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

See [LICENSE-CC-BY](https://github.com/AIMOverse/amico/blob/main/LICENSE-CC-BY)

[![CC BY 4.0][cc-by-shield]][cc-by]
[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg

## Contributing

Contributions are welcome! Please read our [contributing guidelines](https://raw.githubusercontent.com/AIMOverse/amico/main/CONTRIBUTING.md) before submitting a pull request.
