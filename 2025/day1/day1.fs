Create day-1-buf 5 allot

: day-1-part-1 ( fd -- number_of_times_hit_0 )
    >r ( fd on return stack)
    0 50 ( #0 pos )
    begin
        day-1-buf 5 r@ ( #0 pos buf 5 fd )
        read-line throw ( #0 pos #read eof )
    while
        
        ( #0 pos #read -- )
        ( Check if we are going left or right)
        day-1-buf c@ 'L' = ( #0 pos #read =L )
        if
            '-'
        else
            '0'
        endif
        day-1-buf c!
        day-1-buf swap
        ( #0 pos buf #read )
        s>number?
        ( #0 pos delta ddelta success )
        invert
        if
            cr cr
            .S
            cr
            day-1-buf 5 emit
            cr
            abort" Failed conversion"
        endif
        drop
        ( #0 pos change )
        + 100 mod
        dup 0=
        if
            swap 1+ swap ( update #0 )
        endif
        dup .
    repeat
        drop drop
        ( #0 )
    r> drop
;

: day-1-part-2 ( fd -- number_of_times_hit_0 )
    >r ( fd on return stack)
    0 50 ( #0 pos )
    begin
        day-1-buf 5 r@ ( #0 pos buf 5 fd )
        read-line throw ( #0 pos #read eof )
    while
        ( #0 pos #read -- )
        rot >r ( pos #read  R:fd #0 )
        ( Check if we are going left or right)
        day-1-buf c@ 'L' = ( pos #read =L )
        if
            '-'
        else
            '0'
        endif
        day-1-buf c!
        day-1-buf swap
        ( pos buf #read )
        s>number?
        ( pos delta ddelta success )
        invert
        if
            cr cr
            .S
            cr
            day-1-buf 5 emit
            cr
            abort" Failed conversion"
        endif
        drop
        dup .
        ( pos change )
        over 0= -rot
        ( was0 pos change)
        +
        100 /mod
        ( was0 newpos #full_rotations)
        abs r> + >r
        ( was0 newpos )
        dup 0= rot and
        ( newpos was0&is0 )
        if
            r> 1+ >r
        endif
        ( pos )
        r> swap 2dup . .
    repeat
        drop drop
        ( #0 )
    r> drop
;

: test-day-1-part-1
  s" day1.example" r/o open-file throw ( -- fd )
  >r
  r@ day-1-part-1 .
  r>
  close-file throw
;

( Expects to print 1118 )
: do-day-1-part-1
  s" day1.in" r/o open-file throw ( -- fd )
  >r
  r@ day-1-part-1 .
  r>
  close-file throw
;

: test-day-1-part-2
  s" day1.example" r/o open-file throw ( -- fd )
  >r
  r@ day-1-part-2 .
  r>
  close-file throw
;

( Expects to print < 6357 )
: do-day-1-part-2 
  s" day1.in" r/o open-file throw ( -- fd )
  >r
  r@ day-1-part-2 .
  r>
  close-file throw
;