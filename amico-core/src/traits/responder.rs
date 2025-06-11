use crate::types::Interaction;

/// A responder is a core component of the agent which is responsible for
/// responding to interactions.
pub trait Responder {
    /// The response type.
    type Response;

    /// Responds to an interaction.
    fn respond(
        &self,
        interaction: Interaction,
    ) -> impl Future<Output = anyhow::Result<Self::Response>>;
}
