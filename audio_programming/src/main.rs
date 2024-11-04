mod tuning;
mod midi_frequency;
mod sound_files;
mod file_types;
mod scales;

use file_types::file_types::{SoundFileChannelFormat, SoundFileFormat, SoundFileProps, SoundFileSampleType};
use scales::consts_scales::{major_scale, minor_scale};
use sound_files::sound_files::write_wav;

use crate::scales::scales::get_scale;

fn main() {
    let props = SoundFileProps {
        sample_rate: 44100,
        sample_type: SoundFileSampleType::PsfSamp16,
        format: SoundFileFormat::PsfStdWave,
        channel_format: SoundFileChannelFormat::McMono
    };
    _ = write_wav(props, get_scale(major_scale(), 220.0), 1, "major_scale.wav");

    let props = SoundFileProps {
        sample_rate: 44100,
        sample_type: SoundFileSampleType::PsfSamp16,
        format: SoundFileFormat::PsfStdWave,
        channel_format: SoundFileChannelFormat::McMono
    };
    _ = write_wav(props, get_scale(minor_scale(), 220.0), 1, "minor_scale.wav");
}
