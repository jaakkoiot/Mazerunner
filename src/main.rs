mod wavetable_oscillator;

use std::time::Duration;
use rodio::{OutputStream, Source};
use wavetable_oscillator::WavetableOscillator;

fn main() {
    let wavetable_size = 64;
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);

    // Generate sin wavetable
    for n in 0..wavetable_size {
        wavetable.push((2.0 * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin());
    }

    // Initialize WT oscillator
    let mut oscillator = WavetableOscillator::new(44100, wavetable);
    oscillator.set_frequency(337.5);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(5));
}
