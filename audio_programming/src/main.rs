use interactive_exercises::nyquist_frequency::play_nyquist_frequency;

mod tuning;
mod midi_frequency;
mod sound_files;
mod file_types;
mod scales;
mod interactive_exercises;
mod playback;

fn main() { 
   /* let props = SoundFileProps {
        sample_rate: 44100,
        sample_type: SoundFileSampleType::PsfSamp16,
        format: SoundFileFormat::PsfStdWave,
        channel_format: SoundFileChannelFormat::McMono,
        peak_data: ChannelPeakData {
            value: 1.0,
            position: 1
        }
    };
    _ = write_wav(props, get_scale(major_scale(), 220.0), 1, "major_scale.wav");

    let props = SoundFileProps {
        sample_rate: 44100,
        sample_type: SoundFileSampleType::PsfSamp16,
        format: SoundFileFormat::PsfStdWave,
        channel_format: SoundFileChannelFormat::McMono,
        peak_data: ChannelPeakData {
            value: 1.0,
            position: 1
        }
    };
    _ = write_wav(props, get_scale(minor_scale(), 220.0), 1, "minor_scale.wav");
    */
    let _ = play_nyquist_frequency(0., 0.15, 44_100.);
}
