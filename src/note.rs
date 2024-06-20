use crate::*;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Notes {
    notes: Vec<Note>,
    chord: Chord,
    degree: Degree,
    mode: Mode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub key: Key,
    pub accidental: Accidental,
    pub octave: Octave, // TODO:
    pub change: Option<Change>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Change {
    Default,
    Omit,
    Add,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    C = 0,
    // Cs = 1,
    D = 2,
    // Ds = 3,
    E = 4,
    F = 5,
    // Fs = 6,
    G = 7,
    // Gs = 8,
    A = 9,
    // As = 10,
    B = 11,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Octave {
    inner: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Accidental {
    #[default]
    Natural = 0, // ♮
    DoubleFlat = -2, // 𝄫
    Flat = -1,       // ♭
    Sharp = 1,       // ♯
    DoubleSharp = 2, // 𝄪
}

impl Accidental {
    fn as_char(&self) -> char {
        match self {
            Self::Natural => '♮',
            Self::Sharp => '♯',
            Self::Flat => '♭',
            Self::DoubleSharp => '𝄪',
            Self::DoubleFlat => '𝄫',
        }
    }

    pub fn from_i8(v: i8) -> Self {
        use Accidental::*;

        match v {
            0 => Natural,
            1 => Sharp,
            -1 => Flat,
            2 => DoubleSharp,
            -2 => DoubleFlat,
            _ => unreachable!(),
        }
    }
}

impl Notes {
    pub fn new(notes: Vec<Note>, chord: Chord, degree: Degree, mode: Mode) -> Self {
        Self {
            notes,
            chord,
            degree,
            mode,
        }
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn inversion(mut self, idx: ChordIdx) -> Self {
        todo!()
    }

    pub fn omit(&mut self, idx: ChordIdx) {
        let i = idx.as_usize();

        self.notes[i].change = Some(Change::Omit);
    }

    pub fn chord_name(&self) -> Option<String> {
        let leader = format!("{} {}", self.mode, self.degree);
        let root = format!("{}", self.notes()[0].key);
        let quality = self.quality().as_str();
        let chord = self.chord.as_char();

        let mut other = vec![];
        for note in self.notes.iter() {
            let Some(change) = note.change else {
                continue;
            };

            match change {
                Change::Omit => other.push(format!("-{note}")),
                _ => todo!(),
            }
        }

        let other = other.join(",");

        Some(format!("{leader}, {root}{quality}{chord}({other})"))
    }

    pub fn quality(&self) -> Quality {
        let mut flags = vec![];

        for chunk in self.notes().windows(2) {
            let l = &chunk[0];
            let r = &chunk[1];

            let l = l.key as i8 + l.accidental as i8;
            let r = r.key as i8 + r.accidental as i8;

            let semi = if l > r {
                12 % (l - r).abs()
            } else {
                (r - l).abs() % 12
            };

            // dbg!(chunk, semi, (l, r));

            flags.push(semi);
        }

        let part1 = &flags[0..2];

        // dbg!(&flags);
        match self.chord {
            Chord::Triad => match part1 {
                [4, 3] => Quality::Major,
                [3, 4] => Quality::Minor,
                [3, 3] => Quality::Diminished,
                _ => unreachable!(),
            },
            Chord::Seventh => match (part1, flags[2]) {
                ([4, 3], 4) => Quality::Major,
                ([4, 3], 3) => Quality::Dominant,
                ([3, 4], 3) => Quality::Minor,
                ([3, 3], 4) => Quality::HalfDiminished,
                _ => unreachable!(),
            },
            _ => todo!(),
        }
    }
}

impl Note {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            accidental: Accidental::Natural,
            octave: Octave::new(4),
            change: None,
        }
    }

    pub fn interval(&self, rhs: &Note) -> i8 {
        let l = self.key as i8;
        let r = rhs.key as i8;
        let accidental = rhs.accidental as i8 - self.accidental as i8;

        let key = {
            if r > l {
                r - l
            } else {
                1
            }
        };

        key + accidental
    }

    pub fn hz(&self) -> f32 {
        let k = self.key as u8 as f32;
        let o = self.octave.inner as f32;

        let a = 49.0;
        let a_hz = 440.0;
        let fixed = -8.0;

        let n = (12.0 * o) + k + fixed;

        2_f32.powf((n - a) / 12.0) * a_hz
    }
}

impl Octave {
    pub fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub fn as_char(&self) -> char {
        sub_char(char::from_digit(self.inner as u32, 10).unwrap())
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.key.as_char();
        let accidental = self.accidental.as_char();
        let octave = self.octave.as_char();

        write!(f, "{key}{accidental}{octave}")
    }
}

impl Key {
    fn as_char(&self) -> char {
        match self {
            Key::C => 'C',
            Key::D => 'D',
            Key::E => 'E',
            Key::F => 'F',
            Key::G => 'G',
            Key::A => 'A',
            Key::B => 'B',
        }
    }

    pub fn from_u8(v: u8) -> Self {
        use Key::*;

        match v {
            0 => C,
            2 => D,
            4 => E,
            5 => F,
            7 => G,
            9 => A,
            11 => B,
            _ => unreachable!(),
        }
    }

    pub fn next(&self) -> Self {
        todo!()
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::A => 'A',
            Self::B => 'B',
        };

        write!(f, "{c}")
    }
}
