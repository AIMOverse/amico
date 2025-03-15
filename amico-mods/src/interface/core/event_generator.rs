use crate::interface::Plugin;
use amico_core::traits::EventGenerator;

/// The trait for EventGenerator Plugins
pub trait EventGeneratorPlugin: Plugin + EventGenerator {}
