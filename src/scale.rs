use crate::*;

use std::sync::Mutex;

#[derive(Debug)]
pub struct Scale {
    tonic: Note,
    mode: Mode,
    pub inner: [Note; 7],
}

impl Scale {
    pub fn new(mode: Mode, tonic: Note) -> Self {
        let mut tmp = vec![
            Note::new(Key::C),
            Note::new(Key::D),
            Note::new(Key::E),
            Note::new(Key::F),
            Note::new(Key::G),
            Note::new(Key::A),
            Note::new(Key::B),
        ];

        match tonic.key {
            Key::C => tmp.rotate_left(0),
            Key::D => tmp.rotate_left(1),
            Key::E => tmp.rotate_left(2),
            Key::F => tmp.rotate_left(3),
            Key::G => tmp.rotate_left(4),
            Key::A => tmp.rotate_left(5),
            Key::B => tmp.rotate_left(6),
        }

        Self {
            inner: tmp.try_into().unwrap(),
            mode,
            tonic,
        }
        .gen()
    }

    pub fn tonic(&self) -> &Note {
        &self.tonic
    }

    pub fn gen(mut self) -> Self {
        let vec = self.inner.into_iter().map(Mutex::new).collect::<Vec<_>>();

        let mut iter = self.mode.interval().iter();

        for v in vec.windows(2) {
            let (l, mut r) = (v[0].lock().unwrap(), v[1].lock().unwrap());

            let expected = *(iter.next().unwrap()) as i8;
            let interval = l.interval(&*r);
            let accidental = Accidental::from_i8(expected - interval);

            r.accidental = accidental;

            // dbg!(&(l, r, interval, expected, accidental));
        }

        self.inner = {
            let mut res = vec![];

            for v in vec.iter() {
                let mut v = v.lock().unwrap();
                let v_acc = v.accidental as i8;
                let s_acc = self.tonic.accidental as i8;
                let new = v_acc + s_acc;

                v.accidental = Accidental::from_i8(new);

                res.push(v.clone());
            }

            res.try_into().unwrap()
        };

        self
    }

    pub fn chord_by_key(&self, chord: Chord, key: Key) {
        todo!()
    }

    pub fn chord_by_degree(&self, chord: Chord, degree: Degree) -> Notes {
        let v = self.inner.to_vec();
        let list = &degree.list(chord);

        Notes::new(
            list.iter().map(|i| v[*i].clone()).collect(),
            chord,
            degree,
            self.mode,
        )
    }

    pub fn display_line(&self) -> String {
        let mut buf = Vec::with_capacity(7);
        for note in self.inner.iter() {
            buf.push(format!(" {note} "));
        }

        buf.join("")
    }

    pub fn collect_key(&self) -> [Key; 7] {
        self.inner
            .iter()
            .map(|n| n.key)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
