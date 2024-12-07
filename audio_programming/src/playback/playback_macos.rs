use std::f64::consts::PI;

pub struct SineWaveGenerator {
    time: f64,
    /// generated frequency in Hz
    pub freq: f64,
    /// magnitude of generated signal
    volume: f64,
    /// sample rate
    sample_rate: f64
}

impl SineWaveGenerator {
    pub fn new(freq: f64, volume: f64, sample_rate: f64) -> Self {
        SineWaveGenerator {
            time: 0.,
            freq,
            volume,
            sample_rate
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.time += 1. / self.sample_rate;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;
        Some(output)
    }
}
