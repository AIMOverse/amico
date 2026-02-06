# Amico V2 Architecture Design

## Introduction

Amico V2 is a platform-agnostic **runtime** for AI agents built in Rust. It provides a framework for developers to build AI agent business logic similar to how web frameworks like Axum or Rocket enable web development.

This document describes the system architecture using functional design principles, Haskell-style function definitions, and Rust trait examples.

## Design Principles

1. **Traits + Generics over Boxing**: Use traits and generics to describe abstract concepts at high abstraction levels, avoiding dynamic dispatch
2. **Compile-time Safety**: Prefer compile-time types and static evaluation over runtime polymorphism
3. **Zero-cost Abstractions**: Use references and lifetimes over `Box` and `Arc`
4. **Async Traits**: Abstract async tasks with `Future` traits, not `Pin<Box<dyn Future>>`
5. **Modular Architecture**: Organize crates properly, separating core utilities from high-level entry points

## System Layers

The Amico V2 architecture consists of four distinct layers:

```
┌─────────────────────────────────────────┐
│     Application / Event Handlers        │  ← Developer code
├─────────────────────────────────────────┤
│     Workflows Layer (Presets)           │  ← Tool loop agents, etc.
├─────────────────────────────────────────┤
│     Runtime Layer                        │  ← Workflow execution
├─────────────────────────────────────────┤
│     Models Layer                         │  ← Model abstractions
├─────────────────────────────────────────┤
│     System Layer                         │  ← Tools, side-effects, I/O
└─────────────────────────────────────────┘
```

## 1. Models Layer (`amico-models`)

The models layer abstracts away specific AI model providers and parameters, categorizing models by their responsibility.

### Functional Design

```haskell
-- Model categories by capability
type LanguageModel context input output = context -> input -> Future<output>
type ImageGenModel context input = context -> input -> Future<Image>
type VideoGenModel context input = context -> input -> Future<Video>
type SpeechModel context input = context -> input -> Future<Audio>
type EmbeddingModel context input = context -> input -> Future<Vector>

-- Model configuration is abstract
type ModelConfig model = Config model

-- Provider abstraction
type Provider models = Registry models
```

### Rust Trait Design

```rust
/// Core model trait - all models implement this
pub trait Model {
    type Context;
    type Input;
    type Output;
    type Error;

    /// Execute the model with given context and input
    fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a;
}

/// Language model specialization
pub trait LanguageModel: Model<Input = LanguageInput, Output = LanguageOutput> {
    fn with_system_prompt<'a>(
        &'a self,
        prompt: &'a str,
    ) -> impl LanguageModel<
        Context = Self::Context,
        Error = Self::Error,
    > + 'a;
}

/// Image generation model
pub trait ImageGenModel: Model<Input = ImagePrompt, Output = Image> {}

/// Video generation model  
pub trait VideoGenModel: Model<Input = VideoPrompt, Output = Video> {}

/// Speech/Audio model
pub trait SpeechModel: Model<Input = AudioInput, Output = AudioOutput> {}

/// Embedding model
pub trait EmbeddingModel: Model<Input = EmbeddingInput, Output = Vector> {}

/// Model provider trait
pub trait ModelProvider {
    type LanguageModel: LanguageModel;
    type ImageModel: ImageGenModel;
    type SpeechModel: SpeechModel;
    type EmbeddingModel: EmbeddingModel;
    
    fn language_model(&self) -> &Self::LanguageModel;
    fn image_model(&self) -> &Self::ImageModel;
    fn speech_model(&self) -> &Self::SpeechModel;
    fn embedding_model(&self) -> &Self::EmbeddingModel;
}
```

### Key Features

- **Category-based abstraction**: Models are categorized by capability (language, image, video, speech, embedding)
- **Provider agnostic**: Business logic doesn't depend on specific providers (OpenAI, Anthropic, etc.)
- **Composable**: Models can be composed and configured without knowing implementation details
- **Type-safe**: Each model category has specific input/output types

## 2. System Layer (`amico-system`)

The system layer defines how agents interact with the real world through tools and side-effects. This layer is platform-specific and provides interfaces for observing and changing the environment.

### Functional Design

```haskell
-- Tool definition
type Tool input output = input -> Future<output>

-- System capabilities
type FileSystem = FileOperations
type Network = NetworkOperations  
type Process = ProcessOperations

-- Side effect abstraction
type Effect input output = input -> Future<output>

-- Permission system
type Permission resource = Capability<resource>
type PermissionCheck resource = resource -> Bool

-- Observation (sensors)
type Sensor event = Stream<event>
type Observable event = Subscribe<event>
```

### Rust Trait Design

```rust
/// Core tool trait
pub trait Tool {
    type Input;
    type Output;
    type Error;

    /// Execute the tool with given input
    fn execute<'a>(
        &'a self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a;
    
    /// Tool metadata
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// System effect - represents a side effect on the system
pub trait SystemEffect {
    type State;
    type Action;
    type Result;
    type Error;
    
    /// Apply an action to modify system state
    fn apply<'a>(
        &'a mut self,
        action: Self::Action,
    ) -> impl Future<Output = Result<Self::Result, Self::Error>> + 'a;
}

/// Permission system for secure resource access
pub trait Permission<R> {
    fn check(&self, resource: &R) -> bool;
    fn grant(&mut self, resource: R);
    fn revoke(&mut self, resource: &R);
}

/// Observable stream of events (sensors)
pub trait Observable {
    type Event;
    
    /// Subscribe to events
    fn subscribe(&self) -> impl Stream<Item = Self::Event>;
}

/// Platform-specific system interface
pub trait System {
    type FileOps: Tool;
    type NetworkOps: Tool;
    type ProcessOps: Tool;
    
    fn file_ops(&self) -> &Self::FileOps;
    fn network_ops(&self) -> &Self::NetworkOps;
    fn process_ops(&self) -> &Self::ProcessOps;
}
```

### Key Features

- **Platform abstraction**: System interfaces adapt to different platforms (OS, browser, mobile, embedded)
- **Permission model**: Secure, permission-based access to system resources
- **Tool abstraction**: Unified interface for all tools
- **Event streams**: Observable patterns for sensors and environmental changes
- **Side-effect isolation**: Clear separation of pure logic and side effects

## 3. Runtime Layer (`amico-runtime`)

The runtime layer executes workflows on different runtime environments (long-lived or short-lived).

### Functional Design

```haskell
-- Workflow definition
type Workflow context input output = context -> input -> Future<output>

-- Runtime abstraction
type Runtime workflow = Execute<workflow>

-- Runtime types
data RuntimeType = LongLived | ShortLived

-- Execution context
type ExecutionContext = Environment + State + Permissions

-- Runtime scheduler
type Scheduler task = Schedule<task> -> Future<Result>
```

### Rust Trait Design

```rust
/// Workflow trait - defines a unit of work
pub trait Workflow {
    type Context;
    type Input;
    type Output;
    type Error;
    
    /// Execute the workflow
    fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a;
}

/// Execution context for workflows
pub trait ExecutionContext {
    type State;
    type Permissions;
    
    fn state(&self) -> &Self::State;
    fn state_mut(&mut self) -> &mut Self::State;
    fn permissions(&self) -> &Self::Permissions;
}

/// Runtime abstraction
pub trait Runtime {
    type Context: ExecutionContext;
    type Scheduler;
    
    /// Get execution context
    fn context(&self) -> &Self::Context;
    fn context_mut(&mut self) -> &mut Self::Context;
    
    /// Get task scheduler
    fn scheduler(&self) -> &Self::Scheduler;
    
    /// Runtime lifecycle
    fn start(&mut self) -> impl Future<Output = Result<(), RuntimeError>>;
    fn shutdown(&mut self) -> impl Future<Output = Result<(), RuntimeError>>;
}

/// Task scheduler trait
pub trait Scheduler {
    type Task;
    type Handle;
    
    /// Schedule a task for execution
    fn schedule<'a>(
        &'a self,
        task: Self::Task,
    ) -> impl Future<Output = Result<Self::Handle, SchedulerError>> + 'a;
    
    /// Cancel a scheduled task
    fn cancel(&self, handle: Self::Handle) -> Result<(), SchedulerError>;
}

/// Long-lived runtime (like OS processes or Cloudflare Workers)
pub trait LongLivedRuntime: Runtime {
    /// Runtime persists across multiple workflow executions
}

/// Short-lived runtime (like cloud functions)
pub trait ShortLivedRuntime: Runtime {
    /// Runtime exists for single workflow execution
    fn snapshot(&self) -> RuntimeSnapshot;
    fn restore(snapshot: RuntimeSnapshot) -> Self;
}
```

### Key Features

- **Runtime agnostic**: Workflows can run on different runtime types
- **Lifecycle management**: Clear start/shutdown semantics
- **Task scheduling**: Unified scheduler interface (can use tokio, embassy, etc.)
- **State management**: Context and state handling for workflows
- **Persistence**: Support for snapshotting/restoring state in short-lived runtimes

## 4. Workflows Layer (`amico-workflows`)

The workflows layer provides preset workflow patterns and compositions built on top of the runtime layer.

### Functional Design

```haskell
-- Tool loop agent workflow
type ToolLoopAgent model tools context input = 
  context -> input -> Loop (model, tools) -> output

-- Agentic workflow patterns
type ChainOfThought model = model -> Thought -> Action
type ReAct model tools = model -> tools -> Reasoning -> Action -> Observation
type Reflection model = model -> Action -> Critique -> Refinement

-- Multi-agent patterns
type Swarm agents = Broadcast<agents> -> Aggregate<Response>
type Debate agents = agents -> Discussion -> Consensus
```

### Rust Trait Design

```rust
/// Tool loop agent - repeatedly calls tools until goal is met
pub struct ToolLoopAgent<M, T, C> {
    model: M,
    tools: T,
    max_iterations: usize,
    _context: PhantomData<C>,
}

impl<M, T, C> ToolLoopAgent<M, T, C>
where
    M: LanguageModel,
    T: ToolRegistry,
    C: ExecutionContext,
{
    pub fn new(model: M, tools: T, max_iterations: usize) -> Self {
        Self {
            model,
            tools,
            max_iterations,
            _context: PhantomData,
        }
    }
}

impl<M, T, C> Workflow for ToolLoopAgent<M, T, C>
where
    M: LanguageModel,
    T: ToolRegistry,
    C: ExecutionContext,
{
    type Context = C;
    type Input = String;
    type Output = AgentResponse;
    type Error = WorkflowError;
    
    async fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        input: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        // Tool loop implementation
        todo!()
    }
}

/// Chain of thought workflow
pub struct ChainOfThought<M> {
    model: M,
    steps: Vec<ThoughtStep>,
}

/// ReAct (Reasoning + Acting) workflow
pub struct ReActWorkflow<M, T> {
    model: M,
    tools: T,
}

/// Reflection workflow
pub struct ReflectionWorkflow<M> {
    model: M,
    critic: M,
}

/// Multi-agent coordination
pub trait MultiAgentWorkflow {
    type Agent: Workflow;
    type Coordination;
    
    fn agents(&self) -> &[Self::Agent];
    fn coordinate<'a>(
        &'a self,
        responses: Vec<AgentResponse>,
    ) -> impl Future<Output = Self::Coordination> + 'a;
}
```

### Key Features

- **Preset patterns**: Common agent patterns like tool loops, ReAct, chain-of-thought
- **Composable**: Workflows can be composed to create complex behaviors
- **Multi-agent support**: Coordination patterns for multiple agents
- **Reusable**: Built on generic traits, works with any model/tool implementations

## 5. Application Layer (Event Handlers)

The application layer is where developers write their business logic using event handlers - similar to REST endpoint handlers in web frameworks.

### Functional Design

```haskell
-- Event handler signature
type EventHandler event context response = 
  event -> context -> Future<response>

-- Event types
data Event = 
    TimerEvent Time
  | MessageEvent Message
  | BlockchainEvent Transaction
  | SensorEvent SensorData

-- Handler registration
type RegisterHandler event handler = event -> handler -> Runtime
```

### Rust Trait Design

```rust
/// Event trait
pub trait Event {
    fn event_type(&self) -> &str;
    fn timestamp(&self) -> Timestamp;
    fn metadata(&self) -> &EventMetadata;
}

/// Event handler trait
pub trait EventHandler<E: Event> {
    type Context;
    type Response;
    type Error;
    
    /// Handle an event
    fn handle<'a>(
        &'a self,
        event: E,
        context: &'a Self::Context,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + 'a;
}

/// Event router - registers and dispatches events to handlers
pub trait EventRouter {
    type Event: Event;
    type Handler;
    
    /// Register an event handler
    fn register<H>(&mut self, event_type: &str, handler: H)
    where
        H: EventHandler<Self::Event>;
    
    /// Dispatch event to appropriate handler
    fn dispatch<'a>(
        &'a self,
        event: Self::Event,
    ) -> impl Future<Output = Result<(), DispatchError>> + 'a;
}

/// Example: Define a custom event handler
/// 
/// ```rust
/// struct MessageHandler {
///     workflow: ToolLoopAgent<MyModel, MyTools, MyContext>,
/// }
/// 
/// impl EventHandler<MessageEvent> for MessageHandler {
///     type Context = AgentContext;
///     type Response = MessageResponse;
///     type Error = HandlerError;
///     
///     async fn handle(
///         &self,
///         event: MessageEvent,
///         context: &Self::Context,
///     ) -> Result<Self::Response, Self::Error> {
///         let response = self.workflow
///             .execute(context, event.content)
///             .await?;
///         Ok(MessageResponse::from(response))
///     }
/// }
/// ```
```

### Key Features

- **Event-driven architecture**: Handlers respond to various event types
- **Framework-like DX**: Similar developer experience to web frameworks
- **Flexible event sources**: Timer events, messages, blockchain events, sensors, etc.
- **Type-safe handlers**: Compile-time verification of event-handler compatibility

## Entry Point (`amico`)

The main `amico` crate provides the high-level API that combines all layers into a cohesive framework.

### Example Usage

```rust
use amico::{
    models::{LanguageModel, ModelProvider},
    runtime::{Runtime, LongLivedRuntime},
    system::{Tool, System},
    workflows::ToolLoopAgent,
    EventHandler, EventRouter,
};

// Define your model configuration
struct MyModelProvider {
    // ... provider configuration
}

// Define your tools
struct MyTools {
    // ... tool implementations
}

// Define your event handler
struct MyAgentHandler {
    agent: ToolLoopAgent</* types */>,
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

## Crate Organization

```
amico/
├── amico-models/          # Model abstractions (language, image, video, etc.)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── language.rs
│   │   ├── image.rs
│   │   ├── video.rs
│   │   ├── speech.rs
│   │   └── embedding.rs
│   └── Cargo.toml
│
├── amico-system/          # System layer (tools, effects, permissions)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tool.rs
│   │   ├── effect.rs
│   │   ├── permission.rs
│   │   └── observable.rs
│   └── Cargo.toml
│
├── amico-runtime/         # Runtime layer (workflow execution)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── workflow.rs
│   │   ├── context.rs
│   │   ├── scheduler.rs
│   │   └── runtime.rs
│   └── Cargo.toml
│
├── amico-workflows/       # Preset workflows (tool loop, ReAct, etc.)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tool_loop.rs
│   │   ├── react.rs
│   │   ├── reflection.rs
│   │   └── multi_agent.rs
│   └── Cargo.toml
│
└── amico/                 # Main entry crate
    ├── src/
    │   ├── lib.rs
    │   └── event.rs
    └── Cargo.toml
```

## Comparison with Vercel AI SDK

Amico V2 takes inspiration from Vercel's AI SDK architecture:

1. **Model Abstraction**: Like Vercel's provider-agnostic model API, Amico categorizes models by capability
2. **Workflow Support**: Similar to Vercel's workflow SDK for long-running tasks on short-lived functions
3. **Type Safety**: Strong typing throughout the stack
4. **Composability**: Small, composable primitives that can be combined

Key differences:
- **Rust-first**: Zero-cost abstractions, compile-time safety, no GC overhead
- **Platform-agnostic system layer**: Explicit abstraction for different execution environments
- **Trait-based**: Heavy use of traits and generics instead of classes
- **Lifetime-aware**: Explicit lifetime management instead of GC

## Benefits of This Design

1. **Compile-time Safety**: Extensive use of traits and generics catches errors early
2. **Zero-cost Abstractions**: No runtime overhead from abstraction layers
3. **Platform Agnostic**: Works on OS, browsers, mobile, embedded devices
4. **Composable**: Small, focused crates that can be mixed and matched
5. **Developer Friendly**: Framework-like API similar to web frameworks
6. **Performance**: Rust's performance characteristics + no dynamic dispatch
7. **Type Safe**: Compile-time verification of workflow, model, and tool compatibility

## Migration from V1

V1 had some good ideas (event-based architecture, platform-agnostic design) but the implementation was not mature. V2 keeps these concepts while:

- Removing dynamic dispatch in favor of static generics
- Clarifying the separation between models, runtime, system, and workflows
- Providing clearer abstractions inspired by modern frameworks
- Focusing on compile-time safety and zero-cost abstractions

V2 is a complete rewrite with no backward compatibility with V1.
