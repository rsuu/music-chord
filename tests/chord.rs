use music_chord::*;

#[test]
fn test_chord7_quality() {
    use Quality::*;

    let scale = Scale::new(Mode::Ionian, Note::new(Key::C));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [Major, Minor, Minor, Major, Dominant, Minor, HalfDiminished].as_slice(),
        &res
    );

    let scale = Scale::new(Mode::Phrygian, Note::new(Key::E));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [Minor, Major, Dominant, Minor, HalfDiminished, Major, Minor].as_slice(),
        &res
    );

    let scale = Scale::new(Mode::Lydian, Note::new(Key::F));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [Major, Dominant, Minor, HalfDiminished, Major, Minor, Minor].as_slice(),
        &res
    );

    let scale = Scale::new(Mode::Mixolydian, Note::new(Key::G));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [Dominant, Minor, HalfDiminished, Major, Minor, Minor, Major].as_slice(),
        &res
    );

    let scale = Scale::new(Mode::Aeolian, Note::new(Key::A));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [Minor, HalfDiminished, Major, Minor, Minor, Major, Dominant].as_slice(),
        &res
    );

    let scale = Scale::new(Mode::Locrian, Note::new(Key::B));
    let mut res = vec![];
    for i in 0..7 {
        let degree = Degree::from_u8(i);
        let chord = scale.chord_by_degree(Chord::Seventh, degree);

        res.push(chord.quality());
    }
    assert_eq!(
        [HalfDiminished, Major, Minor, Minor, Major, Dominant, Minor].as_slice(),
        &res
    );
}
