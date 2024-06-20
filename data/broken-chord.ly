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
  } \relative c' {
    \info
    \clef G

    %<c e g> <e g c> <g c e>
    %
    % CM3        Inversion
    % c e g      0          CM3/C
    %   e g c    1st        CM3/E
    %     g c e  2nd        CM3/G
    %
    % CM7            Inversion
    % c e g b        0          CM7/C
    %   e g b c      1st        CM7/E
    %     g b c e    2nd        CM7/G
    %       b c e g  3rd        CM7/B


    <g c e>^"CM/G" <c e g> <c e g> |
    <e g b>^"Em/" <e g b> <e g b> |
    <a c e>^"Am/" <a c e> <a c e> |
    <f a c>^"FM/" <f a c> <f a c> |
  }

  % lhs
  \new Staff \with {
    instrumentName = "Piano"
    shortInstrumentName = "Pno."
    midiInstrument = "Grand Piano"
  } \relative {
    \clef F

    c
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
