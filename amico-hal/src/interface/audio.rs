/// Trait for audio playback.
pub trait AudioPlayer {
    /// The error type returned by the `play` method.
    type Error;

    /// Play an audio file.
    fn play(path: &str) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync
    where
        Self: Sized;
}

/// Trait for audio recording.
pub trait AudioRecorder {
    /// The error type returned by the `record` method.
    type Error;

    /// Record audio to a file.
    fn record(path: &str) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync
    where
        Self: Sized;
}
