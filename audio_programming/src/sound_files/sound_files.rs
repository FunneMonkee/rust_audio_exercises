use std::{f64::consts::PI, fs::File, io::{Result, Seek, SeekFrom, Write}, str};
use crate::file_types::file_types::SoundFileProps;

pub fn write_wav(props: SoundFileProps, frequencies: Vec<f64>, duration_per_note: u8, file_name: &str) -> Result<()> {
    let num_channels: u16 = props.channel_format.get_number_of_channels();
    let bits_per_sample: u16 = props.sample_type as u16;
    let byte_rate: u32 = (props.sample_rate * num_channels as u32 * bits_per_sample as u32) / 8; 
    let block_align: u16 = (num_channels * bits_per_sample) / 8;

    let mut file = File::create(file_name)?;

    // ChunkId
    // Contains the letters "RIFF" in ASCII form (0x52494646 big-endian form).
    let _ = file.write_all(&[0x52,0x49,0x46,0x46]);
    //  ---- place holder for file size
    //  ChunkSize 
    //  This is the size of the entire file in bytes minus 8 bytes for the two fields not included in this count:
    //  ChunkID and ChunkSize.
    let file_size_position = file.stream_position()?;
    file.write_all("----".as_bytes())?;
    //  Format
    //  Contains the letters "WAVE" (0x57415645 big-endian form)
    file.write_all(&[0x57,0x41,0x56,0x45])?;

    //  Format
    //  SubChunk1Id 
    //  Contains the letters "fmt " (0x666d7420 big-endian form).
    file.write_all(&[0x66, 0x6d, 0x74, 0x20])?; //extra space to stay 4 bytes long
    // SubChunkId
    // 16 for PCM. This is the size of the rest of the Subchunk which follows this number.
    file.write_all(&16u32.to_le_bytes())?;
    // AudioFormat 
    // PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.
    file.write_all(&1u16.to_le_bytes())?;
    // Num of Channels
    // Mono = 1, Stereo = 2, etc.
    file.write_all(&props.channel_format.get_number_of_channels().to_le_bytes())?;
    // Sample Rate
    // 8000, 44100, etc.
    file.write_all(&props.sample_rate.to_le_bytes())?;
    // Byte Rate
    // == (SampleRate * NumChannels * BitsPerSample) / 8
    file.write_all(&byte_rate.to_le_bytes())?;
    // == (NumChannels * BitsPerSample) / 8
    file.write_all(&block_align.to_le_bytes())?;
    // Bits per sample
    // 8 bits = 8, 16 bits = 16, etc.
    file.write_all(&bits_per_sample.to_le_bytes())?;
    
    // If not PCM then there is space here (compression)
    // SubChunk2Id
    // Contains the letters "data" (0x64617461 big-endian form).
    file.write_all(&[0x64,0x61,0x74,0x61])?;
    // SubChunk2Size
    // (NumSamples * NumChannels * BitsPerSample) / 8 or size of the data following these 4 bytes
    let data_size_position = file.stream_position()?;
    file.write_all("----".as_bytes())?;
    let data_start_position = file.stream_position()?;

    //generate sin wave
    let amplitude: f64 = 0.5;
    let mut angle: f64 = 0.0;
    let samples_required: u64 = props.sample_rate as u64 * duration_per_note as u64;
    
    let mut sample: f64;
    let mut sample_to_write: i16;
    let max_amplitude: f64 = 2.0f64.powi((bits_per_sample as i32 - 1).into()) - 1.0;

    for frequency in frequencies {
        let offset: f64 = 2.0 * PI * frequency/(props.sample_rate as f64);
        for _ in 1..samples_required
        {
            sample = amplitude * angle.sin();
            angle += offset;
            sample_to_write = (sample * max_amplitude) as i16;
            println!("{}",&sample_to_write);
            file.write_all(&sample_to_write.to_le_bytes())?;
        }
    }
    let mut data_end_position = file.stream_position()?;
    
    let chunk_size_data: u32 = (data_end_position - data_start_position) as u32;
    if chunk_size_data % 2 != 0 {
        file.write_all(&[0x00])?;
        data_end_position = file.stream_position()?;
    }
    file.seek(SeekFrom::Start(data_size_position))?;
	
	file.write_all(&chunk_size_data.to_le_bytes())?;
	
    file.seek(SeekFrom::Start(file_size_position))?;
    let chunk_size_header: u32 = (data_end_position - 8) as u32;
    file.write_all(&chunk_size_header.to_le_bytes())?;

    file.sync_all()?;

    Ok(())
}
