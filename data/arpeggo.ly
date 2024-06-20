\version "2.24.3"
\language "english"
\header {
  title = ""
  composer = ""
  arranger = ""
  footnotes = ""
  copyright = "Creative Commons Attribution-ShareAlike 4.0"
}
\noPageBreak

info = {
    \key c \ionian
    \numericTimeSignature
    \time 6/8
    \tempo \markup {
      \italic Allegro
    } 4 = 120
}

main = \new StaffGroup <<
  % rhs
  \new Staff \with {
    instrumentName = "Piano"
    shortInstrumentName = "Pno."
    midiInstrument = "acoustic grand"
  } \relative {
    \info
    \clef G

    g'8 b  d   g d b  |
    g8  b  d   f d b  |
    g8  b  d   e c a  |
    \break

    fs8 a  c   d b g  |
    e8  g  b   c a fs |
    d8  fs a   r r r  |
    \break
  }
>>

\book {
  \paper {

  }
}

\score {
  \main
  \layout {}
}

\score {
  \unfoldRepeats {
    \main
  }
  \midi {}
}
