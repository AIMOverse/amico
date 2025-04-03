use crate::interface::audio::{AudioPlayer, AudioRecorder};

use super::control::{AudioPlaybackError, AudioRecordingError, playback, record_blocking};

pub struct AudioDriver;

impl AudioRecorder for AudioDriver {
    type Error = AudioRecordingError;

    async fn record(path: &str) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        record_blocking(path).await
    }
}

impl AudioPlayer for AudioDriver {
    type Error = AudioPlaybackError;

    async fn play(path: &str) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        playback(path).await
    }
}
