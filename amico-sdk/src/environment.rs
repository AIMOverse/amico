/// A sensor is a component that gets information from the environment.
pub trait Sensor {
    /// The arguments for the sensor.
    type Args;
    /// The result of the sensor.
    type Result;
    /// The error type for the sensor.
    type Error;

    /// Sense the environment.
    fn sense(&self, args: Self::Args) -> Result<Self::Result, Self::Error>;
}

/// An effector is a component that performs actions in the environment.
pub trait Effector {
    /// The arguments for the effector.
    type Args;
    /// The result of the effector.
    type Result;
    /// The error type for the effector.
    type Error;

    /// Perform an action in the environment.
    fn effect(&self, args: Self::Args) -> Result<Self::Result, Self::Error>;
}
