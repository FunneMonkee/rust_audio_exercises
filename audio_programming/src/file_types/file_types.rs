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

pub enum SoundFileFormat {
    PsfFmtUnknown = 0,
    PsfStdwave,
    PsfWaveEx,
    PsfAiff,
    PsfAifc
}

pub enum SoundFileSampleType {
   PsfSampUnknown = 0,
   PsfSamp8,   // not yet supported
   PsfSamp16,
   PsfSamp24,
   PsfSamp32,
   PsfSampIeeeFloat
}

pub enum SoundFileCreationFtype {
    PsfCreateRdwr,
    PsfCreateTemporary, //not implmented, to create private file
    PsfCreateWronly //not implemented, to create private file
}

pub struct SoundFileProps {
    pub sample_rate: u32,
    pub channels: u32,
    pub sample_type: SoundFileSampleType,
    pub format: SoundFileFormat,
    pub channel_format: SoundFileChannelFormat
}
