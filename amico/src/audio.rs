use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use cpal::traits::{HostTrait, StreamTrait};
use lame::Lame;
use rodio::{Decoder, OutputStream};
use rodio::{DeviceTrait, Sink};

pub struct RecordSignal;

pub fn play_blocking(filepath: &str) -> Result<()> {
    // Spawn blocking operation in a separate thread since audio playback uses non-Send types
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(filepath)?);

    // Decode that sound file into a source
    let source = Decoder::new(file)?;

    // _stream must live as long as the sink
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

    Ok(())
}

// Since we can't use lame directly in a thread-safe context, we'll use a simpler approach
// We'll record to a temporary WAV file and then convert it to MP3 after recording
pub fn spawn_record_task(filepath: &str, rx: Receiver<RecordSignal>) -> Receiver<RecordSignal> {
    // Create a temporary WAV file path
    let temp_wav_path = format!("{}.temp.wav", filepath);
    let filepath = filepath.to_string();

    let (task_tx, task_rx) = channel();

    tokio::task::spawn_blocking(move || {
        tracing::debug!("Spawned record task");

        // Record to the temporary WAV file
        record_to_wav(&temp_wav_path, rx).unwrap();

        // Convert WAV to MP3
        convert_wav_to_mp3(&temp_wav_path, &filepath).unwrap();

        // Remove the temporary WAV file
        std::fs::remove_file(&temp_wav_path).unwrap();

        task_tx.send(RecordSignal).unwrap();
    });

    task_rx
}

// Function to record audio to a WAV file
fn record_to_wav(filepath: &str, rx: Receiver<RecordSignal>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    // Ensure directory exists
    if let Some(parent) = Path::new(filepath).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create a buffer to store the recorded samples
    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
    let samples_clone = Arc::clone(&samples);

    let err_fn = |err| eprintln!("Stream error: {}", err);

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let Ok(mut samples) = samples_clone.lock() {
                    samples.extend_from_slice(data);
                }
            },
            err_fn,
            None,
        )
        .expect("Failed to build input stream");

    tracing::info!("Recording...");

    stream.play().unwrap();

    // Wait for the signal
    rx.recv().unwrap();

    tracing::info!("Recording finished.");

    drop(stream);

    // Get the recorded samples
    let samples = Arc::try_unwrap(samples).unwrap().into_inner()?;

    // Write the samples to a WAV file
    let spec = hound::WavSpec {
        channels: channels as u16,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filepath, spec)?;

    for sample in samples {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;

    Ok(())
}

// Function to convert WAV to MP3
fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) -> Result<()> {
    // Read the WAV file
    let mut reader = hound::WavReader::open(wav_path)?;

    let spec = reader.spec();
    let channels = spec.channels as usize;
    let sample_rate = spec.sample_rate;

    // Create the MP3 encoder
    let mut lame = Lame::new().unwrap();

    // Configure the encoder
    lame.set_channels(channels as u8).unwrap();
    lame.set_sample_rate(sample_rate).unwrap();
    lame.set_quality(5).unwrap();
    lame.init_params().unwrap();

    // Create the MP3 file
    let mp3_file = File::create(mp3_path)?;
    let mut mp3_writer = BufWriter::new(mp3_file);

    // Read all samples from the WAV file and convert from f32 to i16
    let samples: Vec<i16> = reader
        .samples::<f32>()
        .filter_map(Result::ok)
        .map(|sample| (sample * 32767.0) as i16) // Convert f32 [-1.0, 1.0] to i16 range
        .collect();

    // Process the samples in chunks
    let chunk_size = 1024 * channels;
    for chunk in samples.chunks(chunk_size) {
        // Split the interleaved samples into left and right channels
        let mut left = Vec::with_capacity(chunk.len() / channels);
        let mut right = Vec::with_capacity(chunk.len() / channels);

        for i in (0..chunk.len()).step_by(channels) {
            left.push(chunk[i]);
            right.push(if channels > 1 { chunk[i + 1] } else { chunk[i] });
        }

        // Encode to MP3
        let mut mp3_buffer = vec![0u8; chunk.len() * 4]; // Allocate enough space for MP3 data
        let encoded_size = lame.encode(&left, &right, &mut mp3_buffer).unwrap();

        // Write the MP3 data
        if encoded_size > 0 {
            mp3_writer.write_all(&mp3_buffer[..encoded_size])?;
        }
    }

    // Flush the MP3 encoder
    let mut mp3_buffer = vec![0u8; 7200]; // Buffer for flush data
    let encoded_size = lame.encode(&[], &[], &mut mp3_buffer).unwrap();

    if encoded_size > 0 {
        mp3_writer.write_all(&mp3_buffer[..encoded_size])?;
    }

    // Flush the file writer
    mp3_writer.flush()?;

    Ok(())
}
