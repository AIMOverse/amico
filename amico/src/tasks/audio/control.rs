use cpal::traits::{HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use rodio::{Decoder, OutputStream};
use rodio::{DeviceTrait, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

#[derive(Debug, thiserror::Error)]
pub enum AudioPlaybackError {
    #[error("Failed to create audio file")]
    CreateError(#[from] std::io::Error),

    #[error("Failed to create output stream")]
    StreamError(#[from] rodio::StreamError),

    #[error("Failed to decode audio file")]
    DecodeError(#[from] rodio::decoder::DecoderError),

    #[error("Failed to play audio source")]
    PlayError(#[from] rodio::PlayError),

    #[error("Failed to join playback handle")]
    SpawnError(#[from] tokio::task::JoinError),
}

pub async fn playback(filepath: &str) -> Result<(), AudioPlaybackError> {
    let filepath = filepath.to_string();
    // Spawn blocking operation in a separate thread since audio playback uses non-Send types
    tokio::task::spawn_blocking(move || {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) =
            OutputStream::try_default().map_err(|err| AudioPlaybackError::StreamError(err))?;

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(
            File::open(filepath).map_err(|err| AudioPlaybackError::CreateError(err))?,
        );

        // Decode that sound file into a source
        let source = Decoder::new(file).map_err(|err| AudioPlaybackError::DecodeError(err))?;

        // _stream must live as long as the sink
        let sink =
            Sink::try_new(&stream_handle).map_err(|err| AudioPlaybackError::PlayError(err))?;
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();

        Ok(())
    })
    .await
    .map_err(AudioPlaybackError::SpawnError)?
}

#[derive(Debug, thiserror::Error)]
pub enum AudioRecordingError {
    #[error("Failed to join recording handle")]
    SpawnError(#[from] tokio::task::JoinError),

    #[error("Failed to create WAV writer")]
    WavWriterError(#[from] hound::Error),

    #[error("Failed to write sample")]
    SampleWriteError,

    #[error("Failed to finalize WAV file")]
    FinalizationError,
}

// A wrapper struct that doesn't need to implement Debug
struct WavWriterWrapper {
    writer: WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl WavWriterWrapper {
    fn write_sample(&mut self, sample: f32) -> Result<(), AudioRecordingError> {
        self.writer
            .write_sample(sample)
            .map_err(|_| AudioRecordingError::SampleWriteError)
    }

    fn finalize(self) -> Result<(), AudioRecordingError> {
        self.writer
            .finalize()
            .map_err(|_| AudioRecordingError::FinalizationError)
    }
}

pub fn record_blocking(filepath: &str) -> Result<(), AudioRecordingError> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    // Create a proper WAV file with the correct format
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(filepath).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let writer = WavWriter::create(filepath, spec)?;
    let writer = WavWriterWrapper { writer };
    let writer = Arc::new(Mutex::new(writer));

    let err_fn = |err| eprintln!("Stream error: {}", err);
    let writer_clone = Arc::clone(&writer);

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let Ok(mut writer) = writer_clone.lock() {
                    for &sample in data {
                        if writer.write_sample(sample).is_err() {
                            eprintln!("Error writing sample");
                            break;
                        }
                    }
                }
            },
            err_fn,
            None,
        )
        .expect("Failed to build input stream");

    println!("Recording for 3 seconds...");
    stream.play().unwrap();

    // Use a blocking sleep instead of tokio's async sleep
    std::thread::sleep(std::time::Duration::from_secs(3));

    drop(stream);

    // Ensure the writer is properly finalized
    match Arc::try_unwrap(writer) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(writer) => writer.finalize()?,
            Err(_) => return Err(AudioRecordingError::FinalizationError),
        },
        Err(_) => return Err(AudioRecordingError::FinalizationError),
    }

    Ok(())
}
