% https://lsr.di.unimi.it/LSR/Search?q=guitar

arrUp = _\markup \translate #'(-.2 . 0) \rotate #90 \char ##x279B
arrDown = _\markup \translate #'(-.2 . 0) \rotate #-90 \char ##x279B

music = \relative c {
 \time 4/4
 \set Timing.beamExceptions = #'()
 \set Timing.baseMoment = #(ly:make-moment 1/8)
 \set Timing.beatStructure = 3,3,2
 d8\arrDown g\arrDown b\arrUp d,\arrDown g\arrDown b\arrUp d,\arrDown g\arrUp |
 f8\arrDown b\3\arrDown  d\arrUp f,\arrDown b\3\arrDown  d\arrUp f,\arrDown b\3\arrUp |
 c8\arrUp g\arrUp e\arrDown c'8\arrUp g\arrUp e\arrDown c'8\arrUp g\arrDown |
 b8\arrUp g\arrUp d\arrDown b'\arrUp g\arrUp d\arrDown b'\arrUp g\arrUp |
}


\score {
 \new StaffGroup <<
   \new Staff <<
     \context Voice { \clef "G_8" \music }
   >>
   \new TabStaff  <<
     \context TabVoice { \clef "moderntab" \music }
   >>
 >>
 \layout {
   \context {
     \Score
     % vertically align the arrows and add padding from staff
     \override TextScript.padding = #3
   }
   \context {
     \Staff
     \override StringNumber.stencil = ##f
   }
 }
}
