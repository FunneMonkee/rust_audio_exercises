mod tuning;
mod midi_frequency;
mod sound_files;
mod file_types;

use sound_files::sound_files::write_wav;

fn main() {
    _ = write_wav();
}
