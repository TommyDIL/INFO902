use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use picovoice::{
    porcupine::{Porcupine, PorcupineBuilder},
    rhino::RhinoInference,
    PicovoiceBuilder,
};

const ACCESS_KEY: &str = "ACCESS_KEY";
const KEYWORD_FILE_PATH: &str = "KEYWORD_FILE_PATH";
const MODEL_PATH: &str = "MODEL_PATH";

fn next_audio_frame() -> Vec<i16> {
    // Define the sample format and buffer size
    let _sample_format = cpal::SampleFormat::I16;
    let buffer_size = 512; // Adjust the buffer size as needed

    // Get the default input device
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input format");

    // Create a buffer to store the audio data
    let buffer: Vec<i16> = vec![0; buffer_size];
    let mut buffer_clone = buffer.clone();

    // Create a stream
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[i16], _: &_| {
                // Copy the audio data to the buffer
                buffer_clone.copy_from_slice(data);
            },
            |err| eprintln!("Error in stream: {}", err),
            None,
        )
        .expect("Failed to build input stream");

    // Start the stream
    stream.play().expect("Failed to start stream");

    buffer
}

fn main() {
    let porcupine: Porcupine =
        PorcupineBuilder::new_with_keyword_paths(ACCESS_KEY, &[KEYWORD_FILE_PATH])
            .model_path(MODEL_PATH)
            .init()
            .expect("Unable to create Porcupine");

    loop {
        let audio_frame: Vec<i16> = next_audio_frame();
        if let Ok(keyword_index) = porcupine.process(&audio_frame) {
            if keyword_index == 0 {
                // `porcupine` detected
            } else if keyword_index == 1 {
                // `bumblebee` detected
            }
        }
    }
}
