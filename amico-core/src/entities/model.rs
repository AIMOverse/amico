/// Trait representing a model that can predict and sense the environment.
pub trait Model {
    /// Predicts the environment.
    fn predict_environment(&self);

    /// Senses the environment.
    fn sense_environment(&self);
}
