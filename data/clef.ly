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
    \time 4/4
    \tempo \markup {
      \italic Allegro
    } 4 = 100
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


    c' d e f g^"G clef" a b

  }

  % lhs
  \new Staff \with {
    instrumentName = "Piano"
    shortInstrumentName = "Pno."
    midiInstrument = "Grand Piano"
  } \relative {
    \clef F


    c d e f^"F clef" g a b

  }

  % lhs
  \new Staff \with {
    instrumentName = "Piano"
    shortInstrumentName = "Pno."
    midiInstrument = "Grand Piano"
  } \relative {
    \clef C


    c'^"C clef" d e f g a b

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
