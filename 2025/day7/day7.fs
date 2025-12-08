require ../common/common.fs

21000 CONSTANT day-7-buf-size
CREATE day-7-buf day-7-buf-size allot
VARIABLE day-7-line-width
VARIABLE day-7-line-count
VARIABLE day-7-start-column
VARIABLE day-7-timelines

: parse-input ( fd -- ) \ initializes variables
    day-7-buf day-7-buf-size rot read-file throw
    ( length )
    day-7-buf
    begin
        ( length ptr )
        dup c@ dup 10 <>
    while
        ( length ptr char)
        'S' = if
            dup day-7-buf - day-7-start-column !
        endif
        1+
    repeat
    assert( 10 = )
    day-7-buf - 1+ dup day-7-line-width !
    ( length width )
    / day-7-line-count !
;

: ptr-at ( col row -- i )
    day-7-line-width @ * + day-7-buf +
;

: value-at ( col row -- c )
    ptr-at c@
;

: splitter-at? ( col row -- bool )
    \ S" splitter-at?" type .S cr
    value-at '^' =
;

: print-grid ( -- )
    day-7-buf day-7-line-width @ day-7-line-count @ * type cr cr
;

: count-splits ( -- answer )
    \ because splits into the same cell don't count, we should do it one row at a time
    0
    day-7-line-count @ 1 DO
        ( acc | i is row )
        
        day-7-start-column @ i 1- + 1+ day-7-line-width @ min
        day-7-start-column @ i 1- - 0 max
        \ S" outer loop" type .S cr key drop
        DO
            ( acc | i is column, j is row)
            \ check if a beam is above
            i j 1- 
            \ S" inner loop" type .S cr 
            value-at 'S' =
            if
                \ S" Incoming beam" type i j .S 2drop cr
                \ There is a beam above, this is either going to be a beam or a splitter
                i j splitter-at? if
                    \ we hit this splitter, increment the count, beams go adjacent
                    1+
                    'S' i 1- j ptr-at 2dup c! 2 + c!
                else
                    \ no splitter, beam continues
                    'S' i j ptr-at c!
                endif
            endif
        LOOP
        \ S" End of outer loop" type .S cr
        \ print-grid
        
    LOOP
;

: day-7-part-1 ( fd -- answer )
    parse-input
    count-splits
;

: prev-timeline-ptr ( col -- ptr )
    day-7-line-width @ + cells day-7-timelines @ +
;

: this-timeline-ptr ( col -- ptr )
    cells day-7-timelines @ +
;

: print-timelines ( -- )
    S" Timelines: " type
    day-7-line-width @ 0 do
        i this-timeline-ptr @ i prev-timeline-ptr @
        i . ':' emit . '>' emit .
    loop
    cr
;

: copy-timelines-to-previous ( -- )
    0 this-timeline-ptr 0 prev-timeline-ptr day-7-line-width @ cells
    move
    0 this-timeline-ptr day-7-line-width @ cells
    erase

    \ print-timelines
;

: count-timelines ( -- answer )
    \ each splitter duplicates timelines, but also we can add them together due to nearby splitters or unsplit inbound
    \ Still can go row by row
    HERE day-7-timelines !
    day-7-line-width @ 2 * cells allot
    0 this-timeline-ptr day-7-line-width @ 2 * cells  erase

    \ day-7-timelines is a next+prev buffer
    \ start with 1 timeline at the start
    1 day-7-start-column @ this-timeline-ptr !

    copy-timelines-to-previous

    day-7-line-count @ 1 DO
        day-7-start-column @ i 1- + 1+ day-7-line-width @ min
        day-7-start-column @ i 1- - 0 max
        \ S" Outer loop" type .S cr key drop
        DO
            ( i is column, j is row )
            i prev-timeline-ptr @ dup 0> if
                ( inbound-timelines )
                i j splitter-at? if
                    \ S" splitting" type .S cr
                    dup i 1- this-timeline-ptr +! i 1+ this-timeline-ptr +!
                else
                    \ S" Simple inbound" type .S cr
                    i this-timeline-ptr +!
                endif
            else
                drop
            endif
        LOOP
        \ print-timelines
        copy-timelines-to-previous
    LOOP

    0 day-7-line-width @ 0 do
        
        i prev-timeline-ptr @ +
    loop
;

: day-7-part-2 ( fd -- answer )
    parse-input
    count-timelines
;

: test-day-7-part-1
  s" day7.example" r/o open-file throw ( -- fd )
  >r
  r@ day-7-part-1 assert( 21 = )
  r>
  close-file throw
;

( Expect to get 1628 )
: do-day-7-part-1
  s" day7.in" r/o open-file throw ( -- fd )
  >r
  r@ day-7-part-1 .
  r>
  close-file throw
;

: test-day-7-part-2
  s" day7.example" r/o open-file throw ( -- fd )
  >r
  r@ day-7-part-2  assert( 40 = )
  r>
  close-file throw
;

( Expect to get 27055852018812 )
: do-day-7-part-2
  s" day7.in" r/o open-file throw ( -- fd )
  >r
  r@ day-7-part-2 .
  r>
  close-file throw
;