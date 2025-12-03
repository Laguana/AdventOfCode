Create day-3-buf 1024 allot

: max 
    2dup > if drop else nip endif
;


: max-in-buf { buf start end -- max_idx max }
    start buf start + C@ '0' -
    start 1+
    ( imax max i )
    begin
        dup end <
    while
        dup
        buf + C@ '0' -
        ( imax max i candidate )
        rot 2dup >
        ( imax i candidate max candidate>max ) if
            drop -rot 
            ( candidate old_imax i)
            nip dup
            ( candidate i i)
            -rot
        else
            -rot drop
        endif
        ( imax max i )
        1+
    repeat
    drop
;

: best-joltage { buf start end -- joltage }
    buf start end 1-
    max-in-buf
    ( idx max )
    10 *
    buf rot 1+ end max-in-buf
    ( max1 idx max2)
    nip +
;

: day-3-part-1 ( fd -- answer ) 
    >r
    0
    begin
        day-3-buf 1024 r@
        read-line throw ( acc #read !eof )
    while
        day-3-buf 0 rot best-joltage +
        ( acc )
    repeat
    drop 
    r> drop
;

: day-3-part-2 0 ;

: test-day-3-part-1
  s" day3.example" r/o open-file throw ( -- fd )
  >r
  r@ day-3-part-1  assert( 357 = )
  r>
  close-file throw
;

( Expect to get 17493 )
: do-day-3-part-1
  s" day3.in" r/o open-file throw ( -- fd )
  >r
  r@ day-3-part-1 .
  r>
  close-file throw
;

: test-day-3-part-2
  s" day3.example" r/o open-file throw ( -- fd )
  >r
  r@ day-3-part-2  . \ assert( 4174379265 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-3-part-2
  s" day3.in" r/o open-file throw ( -- fd )
  >r
  r@ day-3-part-2 .
  r>
  close-file throw
;