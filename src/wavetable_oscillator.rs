// In wavetable_oscillator.rs

use std::time::Duration;
use rodio::{Source, OutputStream};

pub struct WavetableOscillator {
    pub sample_rate: u32,
    pub wavetable: Vec<f32>,
    pub index: f32,
    pub index_increment: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wavetable: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wavetable,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn catmull_rom_interpolation(&self) -> f32 {
        let truncated_index = self.index as usize;
        let len = self.wavetable.len();
    
        // Determine the indices for the four points involved in Catmull-Rom interpolation
        let i0 = (truncated_index + len - 1) % len;
        let i1 = truncated_index;
        let i2 = (truncated_index + 1) % len;
        let i3 = (truncated_index + 2) % len;
    
        // Calculate the interpolation parameter and its powers
        let t = self.index - truncated_index as f32;
        let t2 = t * t;
        let t3 = t2 * t;
    
        // Precompute the weights for Catmull-Rom interpolation
        let a0 = -0.5 * t3 + t2 - 0.5 * t;
        let a1 = 1.5 * t3 - 2.5 * t2 + 1.0;
        let a2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
        let a3 = 0.5 * t3 - 0.5 * t2;
    
        // Perform Catmull-Rom interpolation using precomputed values
        let y0 = self.wavetable[i0];
        let y1 = self.wavetable[i1];
        let y2 = self.wavetable[i2];
        let y3 = self.wavetable[i3];
    
        return a0 * y0 + a1 * y1 + a2 * y2 + a3 * y3;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.catmull_rom_interpolation();
        self.index += self.index_increment;
        self.index %= self.wavetable.len() as f32;
        return sample;
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
