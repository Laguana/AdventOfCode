require ../common/common.fs

CREATE day-10-buf 300 allot

struct
    cell% field machine-desired-state
    cell% field machine-num-lights
    cell% field machine-num-buttons
    cell% field machine-buttons
    \ then afterwards, #lights more cells of desired-power
end-struct machine%

: machine-desired-power ( machine -- ptr )
    dup machine-num-buttons @
    3 + cells +
;

: print-machine { machine -- }
    machine machine-desired-state @ '[' emit . ']' emit
    machine machine-num-lights @ .
    machine machine-num-buttons @ dup .
    0 do
        '(' emit 
        machine machine-buttons i cells + @ .
        ')' emit
    loop
;

: read-bits-list ( pinput -- pinput n )
    0 swap
    begin
        ( acc pinput )
        get-number
        ( acc pinput n )
        1 swap lshift rot + swap
        dup c@ ',' =
    while
        1+
    repeat
    swap
;

: read-machine ( -- machine )
    HERE { machine }
    0 ,
    day-10-buf
    assert( dup c@ '[' = )
    1+ 0 swap
    ( bit-index pinput )
    \ read the desired state
    begin
        dup c@ '#' = if
            swap 1 over lshift machine machine-desired-state +!
            1+ swap 1+
        else
            1+ swap 1+ swap
        endif
        dup c@ ']' =
    until
    swap , 0 ,
    ( pinput )
    1+
    begin
        skip-spaces
        dup c@ '(' =
    while
        1+
        read-bits-list
        ,
        1 machine machine-num-buttons +!
        ( pinput )
        assert( dup c@ ')' = )
        1+
    repeat
    ( pinput )
    assert( dup c@ '{' = )

    begin
        1+
        get-number ,
        dup c@ ',' =
    while
        1+
    repeat
    assert( c@ '}' = )
    machine
;

: solve-machine { machine -- answer }
    \ machine print-machine
    machine machine-num-lights @ 1 swap lshift
    \ there are 2^#lights states we could be in
    dup allocate throw
    { states }
    states swap -1 fill
    0 states c!
    \ we start in state 0, we want to get to the desired state
    \ use the stack as a queue =
    HERE dup
    0 ,
    ( initial-stack reachable-ptr )
    begin
        dup @
        dup machine machine-desired-state @
        \ ." Are we there yet " .S cr
        <>
    while
        ( initial-stack current-ptr current-state )
        dup states + c@ { current-distance }
        machine machine-num-buttons @ 0 do
            \ generate potential next states
            ( initial-stack current-ptr current-state )
            machine machine-buttons i cells + @
            ( initial-stack current-ptr current-state button-effect)
            over xor
            ( initial-stack current-ptr current-state next-state )
            dup states + c@ 
            \ we're doing bfs, if its set then its shorter
            \ ." Is this a new successor?" .S cr
            255 = if
                \ add to the list
                current-distance 1+ over states + c!
                ,
            else
                \ clean up
                drop
            endif
        loop
        drop 
        1 cells +
        assert( dup HERE <> )
    repeat
    2drop dp !
    states machine machine-desired-state @ + c@
    states free throw
;

: day-10-part-1 ( fd -- answer)
    >r
    0
    begin
        day-10-buf 300 r@ read-line throw
        ( acc #read eof? )
    while
        \ ." input: " dup day-10-buf swap type cr
        day-10-buf + 0 swap c!
        HERE >r
        read-machine
        solve-machine +
        r> dp !
    repeat
    drop r> drop
;

: solve-power { machine -- answer }
    0
;

: day-10-part-2 ( fd -- answer)
    >r
    0
    begin
        day-10-buf 300 r@ read-line throw
    while
        day-10-buf + 0 swap c!
        HERE >r
        read-machine
        solve-power +
        r> dp !
    repeat
    drop r> drop
;

: test-day-10-part-1
  s" day10.example" r/o open-file throw ( -- fd )
  >r
  r@ day-10-part-1  assert( 7 = )
  r>
  close-file throw
;

( Expect to get 449 )
: do-day-10-part-1
  s" day10.in" r/o open-file throw ( -- fd )
  >r
  r@ day-10-part-1 .
  r>
  close-file throw
;

: test-day-10-part-2
  s" day10.example" r/o open-file throw ( -- fd )
  >r
  r@ day-10-part-2 . \ assert( 0 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-10-part-2
  s" day10.in" r/o open-file throw ( -- fd )
  >r
  r@ day-10-part-2 .
  r>
  close-file throw
;