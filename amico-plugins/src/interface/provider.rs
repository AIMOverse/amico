use crate::interface::Plugin;
use amico::ai::provider::Provider;

/// The trait for providers of AI models.
/// This plugin should implement the `Provider` trait from the `amico-sdk` crate.
pub trait ProviderPlugin: Provider + Plugin {}
