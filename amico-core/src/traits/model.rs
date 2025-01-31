/// Trait representing a model that can predict and sense the environment.
pub trait Model {
    /// Predicts the environment.
    fn predict_environment(&self);

    /// Perceive the environment.
    fn perceive_environment(&self);
}
