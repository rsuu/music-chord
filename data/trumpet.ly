\version "2.24.3"
\language "english"

Trumpet = {
    \new Staff {
        \set Staff.instrumentName =
            \markup {
                \center-column {
                    "Trumpet" \vspace #-0.3
                    \tiny\concat {"in B"\flat}
                }
            }
        \transposition bf

        \set Staff.midiInstrument = #"bright acoustic"
            c'4 g' c'' g' |
            c'1 |
        \set Staff.midiInstrument = #"trumpet"
            c'4 g' c'' g' |
            R1^"con sord." |
        \set Staff.midiInstrument = #"muted trumpet"
            c'4 g' c''8 8 g'4 |
            R1^"senza sord." |
        \set Staff.midiInstrument = #"trumpet"
            c'4 g' c'' g' |
            c'1 |
    }
}

\score {
    \Trumpet
    %\layout { }
    \midi {
        \context { \Score midiChannelMapping = #'instrument }
    }
}
