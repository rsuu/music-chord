\version "2.24.3"
\language "english"
\header {
  title = ""
  composer = ""
  arranger = ""
  footnotes = ""
}
\noPageBreak

\include "predefined-guitar-fretboards.ly"

info = {
    \key c \ionian
    \numericTimeSignature
    \time 4/4
    \tempo \markup {
      \italic Andante
    } 4 = 90
}

% \storePredefinedDiagram #default-fret-table \chordmode {a}
%                         #ukulele-tuning
%                         #"2-2;1-1;o;o;"

main = {
  \new StaffGroup <<
%    \new ChordNames {
%      \chordmode {
%        c1 | f | g | c
%      }
%    }
%    \new FretBoards {
%      \chordmode {
%        c1 | f | g | c
%      }
%    }
%
    % rhs
    \new Staff \with {
      instrumentName = "Piano"
      shortInstrumentName = "Pno."
      midiInstrument = "Grand Piano"
    } \relative e'' {
      \info
      \clef treble

      r4 ^"Dm7" e16 e8 e16 ~ e d c8 ~ c4   | %m1
      r4 ^"G7"  d16 d8 d16 ~ d c b8 ~ b  a |
      r4 ^"CM7" g16 g8 g16 ~ g a b8 ~ b4   |
      r4 ^"Am7" b16 b8 b16 ~ b c d8 e d    |
      r4 ^"Dm7" e16 e8 e16 ~ e d c8 d a    | \break %m5
    }

    % lhs
    \new Staff \with {
      midiInstrument = "bright acoustic"
    } \relative {
      \clef bass

      %%% `\p`: soft next
      %%% `\f`: forte next
      %%% `cs`: C#
      %%% `cf`: Cb
      %%% `c'`: C4 -> C5
      %%% `c,`: C4 -> C3
      <d  f a c>1 | %m1
      <g, b d f>1 |
      <c  e g b \p>1 |
      <a  c e g>1 | % :/
      <d  f a c>1 | \break %m5
    }

    %\drummode { hh8 8 8 8 8 8 8 8 }
  >>
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
