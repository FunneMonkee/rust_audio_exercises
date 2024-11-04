pub enum Steps {
    Whole,
    Half
}

pub fn major_scale() -> Vec<Steps> {
   Vec::from(
        [
            Steps::Whole,
            Steps::Whole,
            Steps::Half,
            Steps::Whole,
            Steps::Whole,
            Steps::Whole,
            Steps::Half,
        ]
    )
}

pub fn minor_scale() -> Vec<Steps> {
   Vec::from(
        [
            Steps::Whole,
            Steps::Half,
            Steps::Whole,
            Steps::Whole,
            Steps::Half,
            Steps::Whole,
            Steps::Whole,
        ]
    )
}
