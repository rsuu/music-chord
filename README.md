# music-chord

## Usage

```rust
use music_chord::*;

fn main() {
    let tonic = Note::new(Key::B);
    let mode = Mode::Ionian;
    let scale = Scale::new(tonic).set_mode(mode);
    dbg!(&scale.display_line());

    let mut chord = scale.chord_by_degree(Chord::Triad, Degree::VII);
    dbg!(&chord);
}
```

## How does it work?

XD

```text
1. (C) Ionian       :221 2221: C D E F G A B C
2. (D) Dorian       :212 2212: D E F G A B C D
3. (E) Phrygian     :122 2122: E F G A B C D E
4. (F) Lydian       :222 1221: F G A B C D E F
5. (G) Mixolydian   :221 2212: G A B C D E F G
6. (A) Aeolian      :212 2122: A B C D E F G A
7. (B) Locrian      :122 1222: B C D E F G A B

1. Ionian : 221 2221
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1M 2m 3m 4M 5M 6m 7°:
@7: 1M 2m 3m 4M 5  6m 7ø:
    2  2  1  2  2  2  1
C : C  D  E  F  G  A  B :C  D  E  F  G  A  B
D : D  E  F# G  A  B  C#:D  E  F# G  A  B  C#
E : E  F# G# A  B  C# D#:E  F# G# A  B  C# D#
F : F  G  A  Bb C  D  E :F  G  A  Bb C  D  E
G : G  A  B  C  D  E  F#:G  A  B  C  D  E  F#
A : A  B  C# D  E  F# G#:A  B  C# D  E  F# G#
B : B  C# D# E  F# G# A#:B  C# D# E  F# G# A#
    2  2  1  2  2  2  1

2. Dorian : 212 2212
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1m 2m 3M 4M 5m 6° 7M:
@7: 1m 2m 3M 4  5m 6ø 7M:
    2  1  2  2  2  1  2
D : D  E  F  G  A  B  C :D  E  F  G  A  B  C
E : E  F# G  A  B  C# D :E  F# G  A  B  C# D
F : F  G  Ab Bb C  D  Eb:F  G  Ab Bb C  D  Eb
G : G  A  Bb C  D  E  F :G  A  Bb C  D  E  F
A : A  B  C  D  E  F# G :A  B  C  D  E  F# G
B : B  C# D  E  F# G# A :B  C# D  E  F# G# A
C : C  D  Eb F  G  A  Bb:C  D  Eb F  G  A  Bb
    2  1  2  2  2  1  2

3. Phrygian : 122 2122
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1m 2M 3M 4m 5° 6M 7m:
@7: 1m 2M 3  4m 5ø 6M 7m
    1  2  2  2  1  2  2
E : E  F  G  A  B  C  D :E  F  G  A  B  C  D
F : F  Gb Ab Bb C  Db Eb:F  Gb Ab Bb C  Db Eb
G : G  Ab Bb C  D  Eb F :G  Ab Bb C  D  Eb F
A : A  Bb C  D  E  F  G :A  Bb C  D  E  F  G
B : B  C  D  E  F# G  A :B  C  D  E  F# G  A
C : C  Db Eb F  G  Ab Bb:C  Db Eb F  G  Ab Bb
D : D  Eb F  G  A  Bb C :D  Eb F  G  A  Bb C
    1  2  2  2  1  2  2

4. Lydian : 222 1221
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1M 2M 3m 4° 5M 6m 7m:
@7: 1M 2  3m 4ø 5M 6m 7m:
    2  2  2  1  2  2  1
 MM 4     3     4  +  3                    >> 2 11+3
 M     4     3     3  +  4                 >< 2 10+4=14
 mm       3     4     3  +  4              << 2 10+4
 °ø          3     3     4  +  3           <> 1 10+3=13
 MM             4     3     4  +  3        >> 2 11+3
 mm                3     4     3  +  4     << 2 10+4
 mm                   3     4     3  +  3  <= 1 10+3=13
F : F  G  A  B  C  D  E :F  G  A  B  C  D  E
G : G  A  B  C# D  E  F#:G  A  B  C# D  E  F#
A : A  B  C# D# E  F# G#:A  B  C# D# E  F# G#
B : B  C# D# E# F# G# A#:B  C# D# E# F# G# A#
C : C  D  E  F# G  A  B :C  D  E  F# G  A  B
D : D  E  F# G# A  B  C#:D  E  F# G# A  B  C#
E : E  F# G# A# B  C# D#:E  F# G# A# B  C# D#
    2  2  2  1  2  2  1

5. Mixolydian : 221 2212
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1M 2m 3° 4M 5m 6m 7M:
@7: 1  2m 3ø 4M 5m 6m 7M:
    2  2  1  2  2  1  2
G : G  A  B  C  D  E  F :G  A  B  C  D  E  F
A : A  B  C# D  E  F# G :A  B  C# D  E  F# G
B : B  C# D# E  F# G# A :B  C# D# E  F# G# A
C : C  D  E  F  G  A  Bb:C  D  E  F  G  A  Bb
D : D  E  F# G  A  B  C :D  E  F# G  A  B  C
E : E  F# G# A  B  C# D :E  F# G# A  B  C# D
F : F  G  A  Bb C  D  Eb:F  G  A  Bb C  D  Eb
    2  2  1  2  2  1  2

6. Aeolian : 212 2122
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1m 2° 3M 4m 5m 6M 7M:
@7: 1m 2ø 3M 4m 5m 6M 7 :
    2  1  2  2  2  1  2
A : A  B  C  D  E  F  G :A  B  C  D  E  F  G
B : B  C# D  E  F# G  A :B  C# D  E  F# G  A
C : C  D  Eb F  G  Ab Bb:C  D  Eb F  G  Ab Bb
D : D  E  F  G  A  Bb C :D  E  F  G  A  Bb C
E : E  F# G  A  B  C  D :E  F# G  A  B  C  D
F : F  G  Ab Bb C  Db Eb:F  G  Ab Bb C  Db Eb
G : G  A  Bb C  D  Eb F :G  A  Bb C  D  Eb F
    2  1  2  2  2  1  2

7. Locrian : 122 1222
  I 1     3     5     7     9
 II    2     4     6     8     A
III       3     5     7     9     B
 IV          4     6     8     A     C
  V             5     7     9     B     D
 VI                6     8     A     C     E
VII                   7     9     B     D     F
@3: 1° 2M 3m 4m 5M 6M 7m:
@7: 1ø 2M 3m 4m 5M 6  7m:
    1  2  2  1  2  2  2
B : B  C  D  E  F  G  A :B  C  D  E  F  G  A
C : C  Db Eb F  Gb Ab Bb:C  Db Eb F  Gb Ab Bb
D : D  Eb F  Gb Ab Bb C :D  Eb F  Gb Ab Bb C
E : E  F  G  A  Bb C  D :E  F  G  A  Bb C  D
F : F  Gb Ab Bb Cb Db Eb:F  Gb Ab Bb Cb Db Eb
G : G  Ab Bb C  Db Eb F :G  Ab Bb C  Db Eb F
A : A  Bb C  D  Eb F  G :A  Bb C  D  Eb F  G
    1  2  2  1  2  2  2
