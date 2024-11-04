use super::consts_scales::Steps;

pub fn get_scale(scale: Vec<Steps>, original_frequency: f64) -> Vec<f64> {
    assert!(original_frequency > 0.0);
 
    let semitone: f64 = 2f64.powf(12f64.recip()); //approx. 1.0594631
    let mut current_frequency: f64 = original_frequency;
    let mut scale_frequencies: Vec<f64> = Vec::with_capacity(8);

    scale_frequencies.push(current_frequency);
    
    for step in scale {
        current_frequency = match step {
            Steps::Whole => current_frequency * semitone.powf(2.0),
            Steps::Half => current_frequency * semitone
        };

        scale_frequencies.push(current_frequency);
    }
    
    scale_frequencies
}
