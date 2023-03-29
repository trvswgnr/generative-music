use rodio::{OutputStream, Sink, Source};
use std::{f32::consts::PI, time::Duration};

struct Oscillator {
    sample_rate: u32,
    frequency: f32,
    duration: f32,
    wave_type: WaveType,
}

enum WaveType {
    Sine,
    Square,
    Saw,
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.duration <= 0.0 {
            return None;
        }

        self.duration -= 1.0 / self.sample_rate as f32;

        let value = match self.wave_type {
            WaveType::Sine => (self.frequency * 2.0 * PI * self.duration).sin(),
            WaveType::Square => {
                if (self.frequency * 2.0 * PI * self.duration).sin() > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            WaveType::Saw => todo!(),
        };

        Some(value)
    }
}

impl Source for Oscillator {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let (stream, stream_handle) =
        OutputStream::try_default().expect("Couldn't create output stream.");

    let sink = Sink::try_new(&stream_handle).expect("Couldn't create sink.");

    for _ in 0..8 {
        let mut oscillator = Oscillator {
            sample_rate: 44100,
            frequency: 440.0,
            duration: 0.5,
            wave_type: WaveType::Sine,
        };

        let mut silence = Oscillator {
            sample_rate: 44100,
            frequency: 0.0,
            duration: 0.5,
            wave_type: WaveType::Sine,
        };

        sink.append(oscillator);
        sink.append(silence);
    }

    sink.sleep_until_end();
}
