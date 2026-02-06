//! # Amico Plugin System
//!
//! This crate provides the plugin architecture for Amico V2, enabling extensibility
//! across all aspects of the agent lifecycle. Plugins can provide tools, hook into
//! runtime lifecycle events, and extend agent capabilities without modifying core code.
//!
//! ## Design Principles
//!
//! - **Lifecycle-aware**: Plugins hook into startup, shutdown, and event processing
//! - **Capability-based**: Plugins declare what they provide (tools, events, interceptors)
//! - **Composable**: Multiple plugins compose via `PluginSet`
//! - **Zero-cost**: Traits + generics, no dynamic dispatch
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico_plugin::{Plugin, ToolPlugin};
//!
//! struct A2APlugin {
//!     endpoint: String,
//! }
//!
//! impl Plugin for A2APlugin {
//!     type Config = A2AConfig;
//!     type Error = A2AError;
//!
//!     fn name(&self) -> &str { "a2a-connector" }
//!     fn version(&self) -> &str { "1.0.0" }
//!
//!     fn build(config: A2AConfig) -> Result<Self, A2AError> {
//!         Ok(Self { endpoint: config.endpoint })
//!     }
//!
//!     async fn on_start(&mut self) -> Result<(), A2AError> {
//!         // Connect to A2A platform
//!         Ok(())
//!     }
//!
//!     async fn on_shutdown(&mut self) -> Result<(), A2AError> {
//!         // Disconnect from A2A platform
//!         Ok(())
//!     }
//! }
//! ```

use std::future::Future;

/// Core plugin trait - all plugins implement this.
///
/// A plugin represents a reusable extension that hooks into the agent lifecycle.
/// Plugins are initialized with configuration, started with the runtime, and
/// shut down when the runtime stops.
pub trait Plugin {
    /// Plugin configuration type
    type Config;

    /// Error type for plugin operations
    type Error;

    /// Plugin name (used for identification and debugging)
    fn name(&self) -> &str;

    /// Plugin version
    fn version(&self) -> &str;

    /// Build the plugin from configuration
    fn build(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Called when the runtime starts - initialize connections, resources, etc.
    fn on_start(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Called when the runtime shuts down - clean up resources
    fn on_shutdown(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// Plugin that provides tools to the agent.
///
/// A `ToolPlugin` extends the agent's capabilities by making additional tools
/// available. For example, an A2A connector plugin might provide tools for
/// sending messages to other agents on a collaboration platform.
pub trait ToolPlugin: Plugin {
    /// The tool type provided by this plugin
    type ProvidedTool: amico_system::Tool;

    /// Returns the tools provided by this plugin
    fn provided_tools(&self) -> &[Self::ProvidedTool];
}

/// Plugin that provides a model service.
///
/// A `ModelPlugin` provides concrete model implementations (e.g. OpenAI,
/// Anthropic, local models) as a plugin. The framework defines the unified
/// model interface; plugins supply the actual implementations.
///
/// This follows the Vercel AI SDK pattern where model services live in
/// separate packages, not in the framework itself.
pub trait ModelPlugin: Plugin {
    /// The model type provided by this plugin
    type ProvidedModel: amico_models::Model;

    /// Returns a reference to the model provided by this plugin
    fn provided_model(&self) -> &Self::ProvidedModel;
}

/// A composable set of plugins with unified lifecycle management.
///
/// `PluginSet` allows multiple plugins to be composed and their lifecycles
/// managed together. The runtime calls `start_all` and `shutdown_all` to
/// drive all plugins through their lifecycle phases.
pub trait PluginSet {
    /// Error type for set operations
    type Error;

    /// Start all plugins in the set
    fn start_all(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Shutdown all plugins in the set
    fn shutdown_all(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// Empty plugin set - no plugins loaded.
impl PluginSet for () {
    type Error = PluginError;

    async fn start_all(&mut self) -> Result<(), PluginError> {
        Ok(())
    }

    async fn shutdown_all(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// `PluginSet` implementation for a single plugin.
impl<P: Plugin + Send> PluginSet for (P,) {
    type Error = P::Error;

    async fn start_all(&mut self) -> Result<(), P::Error> {
        self.0.on_start().await
    }

    async fn shutdown_all(&mut self) -> Result<(), P::Error> {
        self.0.on_shutdown().await
    }
}

/// A runtime that supports plugins.
///
/// Extends the base `Runtime` trait with plugin management. The runtime
/// is responsible for driving the plugin lifecycle alongside its own.
pub trait PluginRuntime: amico_runtime::Runtime {
    /// The set of plugins managed by this runtime
    type Plugins: PluginSet;

    /// Get a reference to the plugin set
    fn plugins(&self) -> &Self::Plugins;

    /// Get a mutable reference to the plugin set
    fn plugins_mut(&mut self) -> &mut Self::Plugins;
}

/// Plugin error types
#[derive(Debug)]
pub enum PluginError {
    /// Plugin failed to initialize from configuration
    InitializationFailed(String),
    /// Plugin failed to start
    StartupFailed(String),
    /// Plugin failed to shut down
    ShutdownFailed(String),
    /// A plugin operation failed
    OperationFailed(String),
    /// Any other plugin error
    Other(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitializationFailed(msg) => {
                write!(f, "Plugin initialization failed: {}", msg)
            }
            Self::StartupFailed(msg) => write!(f, "Plugin startup failed: {}", msg),
            Self::ShutdownFailed(msg) => write!(f, "Plugin shutdown failed: {}", msg),
            Self::OperationFailed(msg) => write!(f, "Plugin operation failed: {}", msg),
            Self::Other(msg) => write!(f, "Plugin error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Mock plugin for testing --

    struct MockConfig {
        name: String,
    }

    #[derive(Debug)]
    struct MockError;

    struct MockPlugin {
        plugin_name: String,
        started: bool,
    }

    impl Plugin for MockPlugin {
        type Config = MockConfig;
        type Error = MockError;

        fn name(&self) -> &str {
            &self.plugin_name
        }

        fn version(&self) -> &str {
            "0.1.0"
        }

        fn build(config: MockConfig) -> Result<Self, MockError> {
            Ok(Self {
                plugin_name: config.name,
                started: false,
            })
        }

        async fn on_start(&mut self) -> Result<(), MockError> {
            self.started = true;
            Ok(())
        }

        async fn on_shutdown(&mut self) -> Result<(), MockError> {
            self.started = false;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_plugin_build() {
        let config = MockConfig {
            name: "test-plugin".to_string(),
        };
        let plugin = MockPlugin::build(config).unwrap();
        assert_eq!(plugin.name(), "test-plugin");
        assert_eq!(plugin.version(), "0.1.0");
        assert!(!plugin.started);
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        let config = MockConfig {
            name: "lifecycle-test".to_string(),
        };
        let mut plugin = MockPlugin::build(config).unwrap();

        assert!(!plugin.started);

        plugin.on_start().await.unwrap();
        assert!(plugin.started);

        plugin.on_shutdown().await.unwrap();
        assert!(!plugin.started);
    }

    #[tokio::test]
    async fn test_empty_plugin_set() {
        let mut set = ();
        set.start_all().await.unwrap();
        set.shutdown_all().await.unwrap();
    }

    #[tokio::test]
    async fn test_single_plugin_set() {
        let config = MockConfig {
            name: "set-test".to_string(),
        };
        let plugin = MockPlugin::build(config).unwrap();
        let mut set = (plugin,);

        set.start_all().await.unwrap();
        assert!(set.0.started);

        set.shutdown_all().await.unwrap();
        assert!(!set.0.started);
    }

    #[test]
    fn test_plugin_error_display() {
        let err = PluginError::InitializationFailed("bad config".to_string());
        assert_eq!(err.to_string(), "Plugin initialization failed: bad config");

        let err = PluginError::StartupFailed("connection refused".to_string());
        assert_eq!(err.to_string(), "Plugin startup failed: connection refused");

        let err = PluginError::ShutdownFailed("timeout".to_string());
        assert_eq!(err.to_string(), "Plugin shutdown failed: timeout");

        let err = PluginError::OperationFailed("not ready".to_string());
        assert_eq!(err.to_string(), "Plugin operation failed: not ready");

        let err = PluginError::Other("unknown".to_string());
        assert_eq!(err.to_string(), "Plugin error: unknown");
    }
}
