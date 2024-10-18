use crate::file_types::file_types::{SoundFileChannelFormat, SoundFileFormat, SoundFileProps, SoundFileSampleType};

pub fn write_wav_ex() -> () {
    let props = SoundFileProps {
        sample_rate: 1,
        channels: 1,
        sample_type: SoundFileSampleType::PsfSamp24,
        format: SoundFileFormat::PsfWaveEx,
        channel_format: SoundFileChannelFormat::McDolby5_1,
    };
}
