use std::fmt::Display;

const INTERVALS: [[u8; 7]; 7] = [
    [2, 2, 1, 2, 2, 2, 1], // 0
    [2, 1, 2, 2, 2, 1, 2], // 1
    [1, 2, 2, 2, 1, 2, 2], // 2
    [2, 2, 2, 1, 2, 2, 1], // 3
    [2, 2, 1, 2, 2, 1, 2], // 4
    [2, 1, 2, 2, 1, 2, 2], // 5
    [1, 2, 2, 1, 2, 2, 2], // 6
];

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Ionian = 0,
    Dorian = 1,
    Phrygian = 2,
    Lydian = 3,
    Mixolydian = 4,
    Aeolian = 5,
    Locrian = 6,
}

impl Mode {
    pub fn interval(&self) -> &'static [u8; 7] {
        &INTERVALS[*self as usize]
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Ionian => "Ionian",
            Self::Dorian => "Dorian",
            Self::Phrygian => "Phrygian",
            Self::Lydian => "Lydian",
            Self::Mixolydian => "Mixolydian",
            Self::Aeolian => "Aeolian",
            Self::Locrian => "Locrian",
        };

        write!(f, "{v}")
    }
}
