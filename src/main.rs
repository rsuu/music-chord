use music_chord::*;

fn main() {
    let mode = Mode::Ionian;
    let tonic = Note::new(Key::B);
    let scale = Scale::new(mode, tonic);
    dbg!(&scale.display_line());

    let mut chord = scale.chord_by_degree(Chord::Triad, Degree::VII);
    dbg!(&chord);

    chord.omit(ChordIdx::Third);
    dbg!(&chord.chord_name());

    for note in chord.notes().iter() {
        dbg!(&note.hz());
    }
}
