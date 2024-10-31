pub enum SoundFileChannelFormat { 
    StdWave,
    McStd,
    McMono,
    McStereo,
    McQuad,
    McLcrs,
    McBfmt,
    McDolby5_1,
    McWaveEx
}

impl SoundFileChannelFormat {
    pub fn get_number_of_channels(&self) -> u16 {
        match self {
            Self::StdWave | Self::McStd => 0,
            Self::McMono => 1,
            Self::McStereo => 2,
            Self::McQuad | Self::McLcrs | Self::McBfmt => 4,
            Self::McDolby5_1 => 6,
            Self::McWaveEx => 8
        }
    }
}

pub enum SoundFileFormat {
    PsfFmtUnknown = 0,
    PsfStdWave,
    PsfWaveEx,
    PsfAiff,
    PsfAifc
}

pub enum SoundFileSampleType { //bits per sample
   PsfSampUnknown = 0,
   PsfSamp8 = 8,   // not yet supported
   PsfSamp16 = 16,
   PsfSamp24 = 24,
   PsfSamp32 = 32,
   PsfSampIeeeFloat = 64 //floating point
}

pub enum SoundFileCreationFtype {
    PsfCreateRdwr,
    PsfCreateTemporary, //not implmented, to create private file
    PsfCreateWronly //not implemented, to create private file
}

pub struct SoundFileProps {
    pub sample_rate: u32,
    pub sample_type: SoundFileSampleType,
    pub format: SoundFileFormat,
    pub channel_format: SoundFileChannelFormat
}
