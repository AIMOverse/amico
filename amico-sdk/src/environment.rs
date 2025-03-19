use async_trait::async_trait;

/// A sensor is a component that gets information from the environment.
#[async_trait]
pub trait Sensor {
    /// The arguments for the sensor.
    type Args: Send + Sync;
    /// The result of the sensor.
    type Result: Send + Sync;
    /// The error type for the sensor.
    type Error: std::error::Error + Send + Sync;

    /// Sense the environment.
    async fn sense(&self, args: Self::Args) -> Result<Self::Result, Self::Error>;
}

/// An effector is a component that performs actions in the environment.
#[async_trait]
pub trait Effector {
    /// The arguments for the effector.
    type Args: Send + Sync;
    /// The result of the effector.
    type Result: Send + Sync;
    /// The error type for the effector.
    type Error: std::error::Error + Send + Sync;

    /// Perform an action in the environment.
    async fn effect(&self, args: Self::Args) -> Result<Self::Result, Self::Error>;
}
