use music_chord::*;

#[test]
fn test_ionian() {}

#[test]
fn test_dorian() {}

#[test]
fn test_mixolydian() {
    assert_eq!(
        " C♮₄  D♮₄  E♮₄  F♮₄  G♮₄  A♮₄  B♭₄ ",
        Scale::new(Mode::Mixolydian, Note::new(Key::C)).display_line()
    );

    assert_eq!(
        " D♮₄  E♮₄  F♯₄  G♮₄  A♮₄  B♮₄  C♮₄ ",
        Scale::new(Mode::Mixolydian, Note::new(Key::D)).display_line()
    );

    assert_eq!(
        " E♮₄  F♯₄  G♯₄  A♮₄  B♮₄  C♯₄  D♮₄ ",
        Scale::new(Mode::Mixolydian, Note::new(Key::E)).display_line()
    );
}

#[test]
fn test_aeolian() {
    assert_eq!(
        " F♮₄  G♮₄  A♭₄  B♭₄  C♮₄  D♭₄  E♭₄ ",
        Scale::new(Mode::Aeolian, Note::new(Key::F)).display_line()
    );
}

#[test]
fn test_locrian() {
    assert_eq!(
        " F♮₄  G♭₄  A♭₄  B♭₄  C♭₄  D♭₄  E♭₄ ",
        Scale::new(Mode::Locrian, Note::new(Key::F)).display_line()
    );
}
