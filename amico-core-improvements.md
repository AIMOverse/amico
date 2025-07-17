# Amico Core Improvements Proposal

## Executive Summary

This document proposes comprehensive improvements to the `amico-core` crate to address the following key requirements:

1. **Remove `evenio` dependency** and implement a custom, statically-dispatched event system
2. **Improve System handlers** to be async and provide better execution status reporting
3. **Enhance the Strategy API** to be more developer-friendly and focused on AI agent development needs
4. **Ensure no-std compatibility** for future embedded/WASM adaptations

## Current Architecture Analysis

### Strengths
- Event-driven architecture with clear separation of concerns
- Modular design with traits for extensibility
- WASM compatibility considerations
- Clear distinction between external events (`AgentEvent`) and internal ECS events

### Issues Identified
1. **Heavy ECS dependency**: The current system relies heavily on `evenio` ECS framework, which adds complexity and runtime overhead
2. **Synchronous system handlers**: Current `System` trait is synchronous, limiting async operations
3. **Limited execution visibility**: No way for Strategy to monitor system execution status
4. **Complex event handling**: The current event system uses boxed dispatching through `evenio`
5. **Developer experience**: The API is more focused on ECS concepts than AI agent development patterns

## Proposed Improvements

### 1. Custom Event System Design

#### 1.1 Statically-Dispatched Event System

Replace `evenio` with a custom event system that provides:

```rust
// Core event traits
pub trait Event: Send + Sync + 'static {
    type Response: Send + Sync + 'static = ();
}

pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle(&mut self, event: E) -> Result<E::Response, HandlerError>;
}

// Event bus with static dispatch
pub struct EventBus {
    handlers: TypeMap<Box<dyn EventHandlerTrait>>,
}

impl EventBus {
    pub async fn send<E: Event>(&mut self, event: E) -> Result<E::Response, EventError> {
        // Static dispatch based on event type
    }
    
    pub fn register<E: Event, H: EventHandler<E> + 'static>(&mut self, handler: H) {
        // Register handler with compile-time type checking
    }
}
```

#### 1.2 No-std Compatibility

Design the event system to work in `no_std` environments:

```rust
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
use heapless::FnvIndexMap as HashMap;

// Use const generics for compile-time event type registration
pub struct StaticEventBus<const MAX_HANDLERS: usize> {
    handlers: heapless::Vec<EventHandlerEntry, MAX_HANDLERS>,
}
```

### 2. Improved System Handler API

#### 2.1 Async System Handlers

```rust
#[async_trait]
pub trait AsyncSystem: Send + Sync {
    type Input: Send + Sync + 'static;
    type Output: Send + Sync + 'static;
    type Error: Send + Sync + std::error::Error + 'static;
    
    async fn execute(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str { "" }
}

// System execution context with status tracking
pub struct SystemContext {
    pub system_name: &'static str,
    pub execution_id: u64,
    pub started_at: Instant,
    pub status: SystemStatus,
}

pub enum SystemStatus {
    Pending,
    Running,
    Completed(Duration),
    Failed(String),
}
```

#### 2.2 System Registry with Execution Tracking

```rust
pub struct SystemRegistry {
    systems: HashMap<TypeId, Box<dyn AsyncSystemTrait>>,
    execution_tracker: ExecutionTracker,
}

impl SystemRegistry {
    pub async fn execute_system<S: AsyncSystem>(
        &mut self, 
        input: S::Input
    ) -> Result<SystemExecutionResult<S::Output>, SystemError> {
        let context = self.execution_tracker.start_execution::<S>();
        
        match self.get_system::<S>().execute(input).await {
            Ok(output) => {
                self.execution_tracker.complete_execution(context.execution_id);
                Ok(SystemExecutionResult::Success(output))
            }
            Err(error) => {
                self.execution_tracker.fail_execution(context.execution_id, error.to_string());
                Err(SystemError::ExecutionFailed(error.into()))
            }
        }
    }
    
    pub fn get_execution_status(&self, execution_id: u64) -> Option<&SystemStatus> {
        self.execution_tracker.get_status(execution_id)
    }
    
    pub fn get_system_metrics(&self) -> SystemMetrics {
        self.execution_tracker.get_metrics()
    }
}
```

### 3. Enhanced Strategy API

#### 3.1 Agent-Focused Strategy Interface

```rust
#[async_trait]
pub trait Strategy: Send + Sync {
    type Context: Send + Sync + 'static;
    
    async fn process_event(
        &mut self,
        event: &AgentEvent,
        context: &mut Self::Context,
        executor: &mut SystemExecutor,
    ) -> Result<StrategyResult, StrategyError>;
    
    fn create_context(&self) -> Self::Context;
}

pub struct StrategyResult {
    pub response: Option<String>,
    pub actions: Vec<AgentAction>,
    pub should_continue: bool,
}

pub enum AgentAction {
    ExecuteSystem {
        system_type: TypeId,
        input: Box<dyn Any + Send + Sync>,
        priority: ActionPriority,
    },
    SendEvent {
        event: Box<dyn Event>,
        target: Option<String>,
    },
    UpdateContext {
        key: String,
        value: serde_json::Value,
    },
}
```

#### 3.2 System Executor with Status Reporting

```rust
pub struct SystemExecutor {
    registry: SystemRegistry,
    active_executions: HashMap<u64, SystemContext>,
}

impl SystemExecutor {
    pub async fn execute<S: AsyncSystem>(
        &mut self, 
        input: S::Input
    ) -> SystemExecutionHandle<S::Output> {
        let handle = self.registry.execute_system::<S>(input).await;
        SystemExecutionHandle::new(handle)
    }
    
    pub fn get_active_executions(&self) -> &HashMap<u64, SystemContext> {
        &self.active_executions
    }
    
    pub fn get_system_metrics(&self) -> SystemMetrics {
        self.registry.get_system_metrics()
    }
}

pub struct SystemExecutionHandle<T> {
    execution_id: u64,
    result: Option<Result<T, SystemError>>,
}

impl<T> SystemExecutionHandle<T> {
    pub fn execution_id(&self) -> u64 {
        self.execution_id
    }
    
    pub fn is_complete(&self) -> bool {
        self.result.is_some()
    }
    
    pub async fn wait(self) -> Result<T, SystemError> {
        // Wait for execution to complete
    }
}
```

### 4. Improved Agent Architecture

#### 4.1 New Agent Structure

```rust
pub struct Agent<S: Strategy> {
    strategy: S,
    context: S::Context,
    executor: SystemExecutor,
    event_bus: EventBus,
    event_sources: Vec<Box<dyn EventSource>>,
    metrics: AgentMetrics,
}

impl<S: Strategy> Agent<S> {
    pub fn new(strategy: S) -> Self {
        let context = strategy.create_context();
        Self {
            strategy,
            context,
            executor: SystemExecutor::new(),
            event_bus: EventBus::new(),
            event_sources: Vec::new(),
            metrics: AgentMetrics::new(),
        }
    }
    
    pub fn add_system<T: AsyncSystem + 'static>(&mut self, system: T) {
        self.executor.register_system(system);
    }
    
    pub fn add_event_source<E: EventSource + 'static>(&mut self, source: E) {
        self.event_sources.push(Box::new(source));
    }
    
    pub async fn run(&mut self) -> Result<(), AgentError> {
        // Main event loop with improved error handling and metrics
        let mut event_stream = self.create_event_stream().await?;
        
        while let Some(event) = event_stream.next().await {
            self.metrics.record_event_received(&event);
            
            match self.strategy.process_event(
                &event, 
                &mut self.context, 
                &mut self.executor
            ).await {
                Ok(result) => {
                    self.handle_strategy_result(result).await?;
                }
                Err(error) => {
                    self.handle_strategy_error(error).await?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn get_metrics(&self) -> &AgentMetrics {
        &self.metrics
    }
    
    pub fn get_system_status(&self) -> SystemStatus {
        self.executor.get_system_metrics()
    }
}
```

### 5. Developer Experience Improvements

#### 5.1 Builder Pattern for Agent Configuration

```rust
pub struct AgentBuilder<S> {
    strategy: Option<S>,
    systems: Vec<Box<dyn AsyncSystemTrait>>,
    event_sources: Vec<Box<dyn EventSource>>,
    config: AgentConfig,
}

impl<S: Strategy> AgentBuilder<S> {
    pub fn new() -> Self {
        Self {
            strategy: None,
            systems: Vec::new(),
            event_sources: Vec::new(),
            config: AgentConfig::default(),
        }
    }
    
    pub fn with_strategy(mut self, strategy: S) -> Self {
        self.strategy = Some(strategy);
        self
    }
    
    pub fn add_system<T: AsyncSystem + 'static>(mut self, system: T) -> Self {
        self.systems.push(Box::new(system));
        self
    }
    
    pub fn add_event_source<E: EventSource + 'static>(mut self, source: E) -> Self {
        self.event_sources.push(Box::new(source));
        self
    }
    
    pub fn with_config(mut self, config: AgentConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn build(self) -> Result<Agent<S>, BuildError> {
        let strategy = self.strategy.ok_or(BuildError::MissingStrategy)?;
        let mut agent = Agent::new(strategy);
        
        for system in self.systems {
            agent.add_system_boxed(system);
        }
        
        for source in self.event_sources {
            agent.add_event_source_boxed(source);
        }
        
        agent.configure(self.config);
        Ok(agent)
    }
}
```

#### 5.2 Macro Support for Common Patterns

```rust
// Macro for defining systems
#[macro_export]
macro_rules! define_system {
    ($name:ident, $input:ty, $output:ty, $body:expr) => {
        pub struct $name;
        
        #[async_trait]
        impl AsyncSystem for $name {
            type Input = $input;
            type Output = $output;
            type Error = SystemError;
            
            async fn execute(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
                $body(input).await
            }
            
            fn name(&self) -> &'static str {
                stringify!($name)
            }
        }
    };
}

// Usage example:
define_system!(
    ProcessTextSystem,
    String,
    ProcessedText,
    |text: String| async move {
        // Process text logic
        Ok(ProcessedText::new(text))
    }
);
```

### 6. Migration Strategy

#### 6.1 Phased Migration Plan

**Phase 1: Event System Replacement**
- Implement custom event system alongside existing `evenio` integration
- Add feature flags for gradual migration
- Maintain backward compatibility

**Phase 2: System Handler Improvements**
- Introduce async system traits
- Add execution tracking and metrics
- Migrate existing systems to new API

**Phase 3: Strategy API Enhancement**
- Implement new Strategy trait
- Add system executor with status reporting
- Improve developer experience with builders and macros

**Phase 4: Final Integration**
- Remove `evenio` dependency
- Complete API migration
- Update documentation and examples

#### 6.2 Compatibility Layer

```rust
// Compatibility layer for existing code
#[cfg(feature = "evenio-compat")]
pub mod compat {
    use super::*;
    
    pub trait LegacySystem {
        fn register_to(self, registry: HandlerRegistry);
    }
    
    impl<T: LegacySystem> From<T> for Box<dyn AsyncSystemTrait> {
        fn from(legacy: T) -> Self {
            Box::new(LegacySystemAdapter::new(legacy))
        }
    }
}
```

## Benefits of Proposed Changes

### 1. Performance Improvements
- **Static dispatch**: Eliminates runtime overhead of dynamic dispatch
- **Reduced dependencies**: Removing `evenio` reduces compile times and binary size
- **Async systems**: Better resource utilization and scalability

### 2. Developer Experience
- **Simplified API**: Focus on AI agent development patterns rather than ECS concepts
- **Better error handling**: Comprehensive error types and reporting
- **Execution visibility**: Real-time system status and metrics
- **Builder pattern**: Easier agent configuration and setup

### 3. Flexibility and Extensibility
- **No-std compatibility**: Enables embedded and resource-constrained environments
- **Modular design**: Easy to add new systems and event sources
- **Type safety**: Compile-time guarantees for event handling

### 4. Maintainability
- **Reduced complexity**: Simpler event system without ECS overhead
- **Better testing**: Async systems are easier to unit test
- **Clear separation**: Distinct layers for events, systems, and strategy

## Implementation Timeline

- **Week 1-2**: Implement custom event system and basic infrastructure
- **Week 3-4**: Develop async system handler framework
- **Week 5-6**: Create enhanced Strategy API and system executor
- **Week 7-8**: Implement developer experience improvements (builders, macros)
- **Week 9-10**: Migration tooling and compatibility layer
- **Week 11-12**: Testing, documentation, and final integration

## Conclusion

These improvements will transform `amico-core` from an ECS-focused framework to a developer-friendly AI agent platform. The proposed changes address all the requirements while maintaining backward compatibility during the migration period. The result will be a more performant, maintainable, and user-friendly framework that better serves AI agent developers.