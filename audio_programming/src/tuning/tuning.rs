use std::f32::consts::PI;

pub fn draw_sin() -> () {
    let n_samples: f32 = 52.0;
    let mut sample: f32;
    let sin_wave_length: f32 = 2.0 * PI;
    let distance_between_two_points: f32 = sin_wave_length / n_samples;

    for i in 0..n_samples as i32 {
        sample = (distance_between_two_points * i as f32).sin();
        println!("{}", sample);
    }
}
//At the standard CD sample rate of 44,100 Hz, our 50-point sine wave will have a frequency of 882 Hz. To obtain the international tuning standard frequency for ‘‘Concert A’’ (440 Hz), we would have to use a sample rate of 22,000 Hz. This is impractical—we need to scale the number of points instead.
pub fn draw_sin_with_rate(
    n_samples: f32,
    frequency: f32,
    rate: f32
) -> () {
    let mut sample: f32;
    let sin_wave_length: f32 = 2.0 * PI;
    let distance_between_two_points: f32 = sin_wave_length * (frequency / rate);

    for i in 0..n_samples as i32 {
        sample = (distance_between_two_points * i as f32).sin();
        println!("{}", sample);
    }
}
//tuning fork v1, exponential decay -> x = ae^(-k/T)
pub fn tuning_fork_v1(
    duration: f32,
    frequency: f32,
    rate: f32,
    slope: f32,
) -> () {
    let n_samples: f32 = duration * rate;
    let sin_wave_length: f32 = 2.0 * PI;
    let distance_between_two_points: f32 = sin_wave_length * (frequency / rate);
    let k: f32 = duration / n_samples;
    let a: f32 = (-k / slope).exp();
    let mut max_amplitude: f32 = 1.0;
    let mut sample: f32;

    for i in 0..n_samples as i32 {
        sample = (distance_between_two_points * i as f32).sin();
        max_amplitude = max_amplitude * a;
        sample = sample * max_amplitude;
        println!("{}", sample);
    }
}
//tuning fork v2
pub fn tuning_fork_v2(
    duration: f32,
    frequency: f32,
    rate: f32,
    amplitude: f32
) -> () {
    let n_samples: f32 = duration * rate;
    let sin_wave_length: f32 = 2.0 * PI;
    let distance_between_two_points: f32 = sin_wave_length * (frequency / rate);
    let mut start: f32 = 1.0;
    let end: f32 = 1.0 * -4.0_f32.exp(); // -80db
    let mut max_sample: f32 = 0.0;
    let factor = (end / start).powf(1.0 / n_samples);
    let mut sample: f32;

    for i in 0..n_samples as i32 {
        sample = amplitude * (distance_between_two_points * i as f32).sin();
        sample = sample * start;
        start = start * factor;
        println!("{}", sample);
        max_sample = max_sample.max(sample.abs());
    }
}
