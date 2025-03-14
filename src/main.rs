use picovoice::{
    porcupine::{Porcupine, PorcupineBuilder},
    rhino::RhinoInference,
    PicovoiceBuilder,
};

const ACCESS_KEY: &str = "ACCESS_KEY";
const KEYWORD_FILE_PATH: &str = "KEYWORD_FILE_PATH";
const MODEL_PATH: &str = "MODEL_PATH";

fn main() {
    let wake_word_callback = || {
        // let user know wake word detected
    };
    let inference_callback = |inference: RhinoInference| {
        if inference.is_understood {
            let intent = inference.intent.unwrap();
            let slots = inference.slots;
            // add code to take action based on inferred intent and slot values
        } else {
            // add code to handle unsupported commands
        }
    };

    let porcupine: Porcupine =
        PorcupineBuilder::new_with_keyword_paths(ACCESS_KEY, &[KEYWORD_FILE_PATH])
            .model_path(MODEL_PATH)
            .init()
            .expect("Unable to create Porcupine");

    println!("Hello, world!");
}
