use coreaudio::audio_unit::{render_callback::{self, data}, AudioUnit, IOType, SampleFormat};

use crate::playback::playback_macos::SineWaveGenerator;

// nyquist frequency ≈ sample_rate / 2
// when frequency reaches nyquist frequency it will basically sound backwards
// aliased frequency = | frequency - sample_rate * (frequency / sample_rate).rounded_up |
pub fn play_nyquist_frequency(starting_freq: f64, volume: f64, sample_rate: f64) -> Result<(), coreaudio::Error> {
    let mut current_frequency = starting_freq;
    let mut actual_frequency = current_frequency;
    let mut samples = SineWaveGenerator::new(current_frequency, volume, sample_rate);
    let nyquist_frequency: f64 = sample_rate / 2.;
    // construct an Output audio unit that delivers audio to the default output device.
    let mut audio_unit = AudioUnit::new(IOType::DefaultOutput)?;

    // read the input format. This is counterintuitive, but it's the format used when sending
    // audio data to the AudioUnit representing the output device. This is separate from the
    // format the AudioUnit later uses to send the data to the hardware device.
    let stream_format = audio_unit.output_stream_format()?;
    println!("{:#?}", &stream_format);

    // sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    type Args = render_callback::Args<data::NonInterleaved<f32>>;
    audio_unit.set_render_callback(move |args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;
        for i in 0..num_frames {
            let sample = samples.next().unwrap();
            print!("\r");
            print!("current frequency -> {} aliased frequency -> {}", current_frequency, actual_frequency);
            current_frequency = current_frequency + 0.08;
            samples.freq = current_frequency;

            if current_frequency <= nyquist_frequency {
               actual_frequency = current_frequency;
            }
            else {
               actual_frequency = (current_frequency - sample_rate * (current_frequency / sample_rate).round()).abs();
            }

            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
        Ok(())
    })?;
    audio_unit.start()?;

    std::thread::sleep(std::time::Duration::from_millis(6000)); 

    println!("");
    Ok(())
}
