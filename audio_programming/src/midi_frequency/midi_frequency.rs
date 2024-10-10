//display E.T frequencies for an N-note octave, from a given MIDI note
pub fn get_n_frequencies_for_n_scale_from_base_note(base_note: i32, n: usize) -> Option<Vec<f32>> {
    if 1 >= base_note || base_note >= 127 {
        println!("MIDI base note must be between 1 and 127");
        return None;
    }
    let mut frequencies: Vec<f32> = Vec::with_capacity(n + 1);
    
    let ratio: f32 = get_ratio_for_n_scale(n as f32);
    //initial frequency is the 12 tone equal temperament note 
    let mut frequency = calculate_frequency(base_note);

    for i in 0..n {
        frequencies[i] = frequency;
        frequency = frequency * ratio;
    } 

    Some(frequencies)
}

//scale is r^n = p in equal temperement, so r = p^-n
pub fn calculate_frequency(note: i32) -> f32 {
    assert!(1 <= note && note <= 127);
    
    let semitone_ratio: f32 = get_ratio_for_n_scale(12f32);
    let c5: f32 = 220.0 * semitone_ratio.powf(3.0); //220 -> A / mid c is 3 semitones above
    let c0: f32 = c5 * 0.5f32.powf(5.0); //lowest c note / for each octave lowered divide by 2
    
    c0 * semitone_ratio.powf(note as f32)
}

pub fn get_note_from_frequency(frequency: f32) -> i32 {
    let semitone_ratio: f32 = get_ratio_for_n_scale(12f32);
    let c5: f32 = 220.0 * semitone_ratio.powf(3.0); //220 -> A / mid c is 3 semitones above
    let c0: f32 = c5 * 0.5f32.powf(5.0); //lowest c note / for each octave lowered divide by 2

    let midi: f32 = (frequency / c0).log10() / semitone_ratio.log10();
    midi.round() as i32
}

pub fn get_ratio_for_n_scale(n: f32) -> f32 {
    2f32.powf(n.recip()) //approx. 1.0594631
}
