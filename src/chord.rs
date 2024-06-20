use crate::*;

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Chord {
    Triad = 3,
    Seventh = 4,
    Ninth = 5,
    Eleventh = 6,
    Thirteenth = 7,
    // ?[1,5]
    // PowerChord,
}

// REFS: https://en.wikipedia.org/wiki/Interval_(music)#Quality
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quality {
    // unused
    // Perfect,        // P
    // Augmented,      // A
    Major,          // M
    Minor,          // m
    Diminished,     // °
    Dominant,       // (Empty)
    HalfDiminished, // ø
    Suspended2,     // sus2
    Suspended4,     // sus4
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChordIdx {
    Root = 1,
    Third = 3,
    Fifth = 5,
    Seventh = 7,
    Ninth = 9,
    Eleventh = 11,
    Thirteenth = 13,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Degree {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    VI = 5,
    VII = 6,
}

impl Quality {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Self::Perfect => "P",
            // Self::Augmented => "A",
            Self::Major => "M",
            Self::Minor => "m",
            Self::Diminished => "°",
            Self::Dominant => "",
            Self::HalfDiminished => "ø",
            Self::Suspended2 => "ˢᵘˢ²",
            Self::Suspended4 => "ˢᵘˢ⁴",
        }
    }
}

impl Chord {
    pub fn len(&self) -> usize {
        match self {
            Self::Triad => 3,
            Self::Seventh => 4,
            Self::Ninth => 5,
            Self::Eleventh => 6,
            Self::Thirteenth => 7,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            3 => Self::Triad,
            4 => Self::Seventh,
            5 => Self::Ninth,
            6 => Self::Eleventh,
            7 => Self::Thirteenth,
            _ => unreachable!(),
        }
    }

    pub fn as_char(&self) -> char {
        sup_char(char::from_digit(self.order() as u32, 10).unwrap())
    }

    pub fn order(&self) -> u8 {
        match self {
            Self::Triad => 3,
            Self::Seventh => 7,
            Self::Ninth => 9,
            Self::Eleventh => 11,
            Self::Thirteenth => 13,
        }
    }
}

impl Degree {
    pub fn list(&self, chord: Chord) -> Vec<usize> {
        let offset = *self as usize;
        let max = (Self::VII as usize) + 1;

        (0..chord.len())
            .map(|idx| (offset + idx * 2) % max)
            .collect()
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::I,
            1 => Self::II,
            2 => Self::III,
            3 => Self::IV,
            4 => Self::V,
            5 => Self::VI,
            6 => Self::VII,
            _ => unreachable!(),
        }
    }
}

impl ChordIdx {
    pub fn as_usize(&self) -> usize {
        match *self {
            Self::Root => 0,
            Self::Third => 1,
            Self::Fifth => 2,
            Self::Seventh => 3,
            Self::Ninth => 4,
            Self::Eleventh => 5,
            Self::Thirteenth => 6,
        }
    }
}

impl Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::I => 'Ⅰ',
            Self::II => 'Ⅱ',
            Self::III => 'Ⅲ',
            Self::IV => 'Ⅳ',
            Self::V => 'Ⅴ',
            Self::VI => 'Ⅵ',
            Self::VII => 'Ⅶ',
        };

        write!(f, "{v}")
    }
}
