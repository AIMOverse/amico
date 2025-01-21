/// A trait for types that can be converted into a prompt string for AI/LLM interactions.
///
/// The `Prompt` trait provides a standardized way to convert various types of data or
/// structures into formatted prompt strings that can be used with AI language models.
/// This allows for consistent prompt generation across different parts of the application
/// while maintaining type safety and reusability.
///
/// # Examples
///
/// Basic implementation for a custom type:
/// ```
/// use amico_core::prompt::Prompt;
///
/// struct UserQuery {
///     context: String,
///     question: String,
/// }
///
/// impl Prompt for UserQuery {
///     fn to_prompt(&self) -> String {
///         format!(
///             "Context: {}\nQuestion: {}",
///             self.context,
///             self.question
///         )
///     }
/// }
///
/// let query = UserQuery {
///     context: "The weather is sunny".to_string(),
///     question: "What should I wear?".to_string(),
/// };
///
/// assert_eq!(
///     query.to_prompt(),
///     "Context: The weather is sunny\nQuestion: What should I wear?"
/// );
/// ```
///
/// Implementation for a system message:
/// ```
/// use amico_core::prompt::Prompt;
///
/// struct SystemMessage {
///     role: String,
///     instructions: Vec<String>,
/// }
///
/// impl Prompt for SystemMessage {
///     fn to_prompt(&self) -> String {
///         let formatted_instructions = self.instructions.join("\n- ");
///         format!(
///             "Role: {}\nInstructions:\n- {}",
///             self.role,
///             formatted_instructions
///         )
///     }
/// }
/// ```
///
/// # Implementation Guidelines
///
/// When implementing this trait:
/// - Ensure the generated prompt is clear and well-structured
/// - Consider including relevant context and formatting
/// - Avoid including sensitive or personal information
/// - Keep the output concise while maintaining clarity
///
pub trait Prompt {
    /// Converts the implementing type into a prompt string.
    ///
    /// This method should format the type's data into a string that can be
    /// effectively used as input for an AI/LLM model.
    ///
    /// # Returns
    ///
    /// Returns a [`String`] containing the formatted prompt.
    fn to_prompt(&self) -> String;
}
