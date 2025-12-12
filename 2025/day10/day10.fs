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
    '{' emit 
    machine machine-num-lights @ 0 DO
        machine machine-desired-power i cells + @ .
    LOOP
    '}' emit cr
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
        dup c@ ',' <>
    until
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

struct
    char% field matrix-count-rows
    char% field matrix-count-columns
    cell% drop 0 field matrix-data \ rows * columns entries actually
end-struct matrix%

: print-matrix { matrix -- }
    matrix matrix-data
    matrix matrix-count-rows c@ 0 DO
        '[' emit
        matrix matrix-count-columns c@ 0 DO
            dup @ .
            1 cells +
        LOOP
        ']' emit cr
    LOOP
    drop
;

: matrix-at { matrix row col -- v }
    \ ." m[r,c]" row . col . 
    matrix matrix-count-columns c@
    row * col + cells 
    \ '(' emit dup . ')' emit
    matrix matrix-data + @
    \ '=' emit dup . cr
;

: machine>matrix { machine -- matrix }
    \ see solve-power for description
    \ machine print-machine
    matrix% %allot { matrix } 
    machine machine-num-lights @ dup matrix matrix-count-rows c!
    machine machine-num-buttons @ dup 1+ matrix matrix-count-columns c!
    \ assert( matrix matrix-data HERE .S cr = )
    { #lights #buttons }
    #lights 0 DO
        \ for each button, if it touches that index then 1 else 0
        #buttons 0 DO
            machine machine-buttons i cells + @
            j bit-set? negate 
            \ dup .
            ,
        LOOP
        \ end with the desired count
        machine machine-desired-power i cells + @ 
        \ dup . cr
        ,
    LOOP
    matrix
    \ dup print-matrix
;

: scale-matrix-row { matrix row c -- }
    matrix matrix-count-columns c@ dup
    ( #columns #columns )
    row * cells
    matrix matrix-data +
    swap 0 DO
        ( ptr )
        dup @ c * over !
        cell+
    LOOP
    drop
;

: swap-matrix-rows { matrix r1 r2 -- }
    here dup >r
    matrix matrix-count-columns c@ dup cells allot
    ( tempspace #columns )
    cells
    dup r1 * matrix matrix-data +
    ( tempspace |row| pr1 )
    2dup swap
    ( tempspace |row| pr1 pr1 |row| )
    4 pick swap move
    ( tempspace |row| pr1 )
    over r2 * matrix matrix-data +
    ( tempspace |row| pr1 pr2 )
    -rot
    ( tempspace pr2 |row| pr1 )
    2 pick swap 2 pick
    ( tempspace pr2 |row| pr2 pr1 |row| )
    move move
    r> dp !
;

\ add c * r1 to r2 
: add-matrix-rows { matrix r1 r2 c -- }
    matrix matrix-count-columns c@ dup cells
    ( #columns |row| )
    dup r2 * matrix matrix-data + swap
    r1 * matrix matrix-data +
    ( #columns pr2 pr1 )
    rot 0 DO
        2dup 
        ( pr2 pr1 pr2 pr1 )
        @ c * swap +!
        cell+ swap cell+ swap
    LOOP
    2drop
;

: matrix-row-with-col ( matrix col startrow -- row )
    >r
    over matrix-count-rows c@ r> DO
        2dup i swap matrix-at 0<> if
            ( matrix col )
            2drop i unloop exit
        endif
    LOOP
    2drop -1
;

: matrix-row-starts-negative { matrix row -- f }
    matrix matrix-count-columns c@ cells row *
    matrix matrix-data +
    matrix matrix-count-columns c@ 0 do
        ( ptr )
        dup @ ?dup 0<> if
            nip unloop 0< exit
        endif
        cell+
    loop
    drop false
;

: gaussian-eliminate { matrix -- } \ modify in place
    0
    matrix matrix-count-rows c@ 1- 0 DO
        \ ." Eliminating for row " i . .S cr
        \ take the leading value in each row
        \ put it at the top
        \ subtract from cells below to 0 them outcome
        ( column )
        begin 
            matrix over i matrix-row-with-col 
            ( column row-with-col )
            \ ." row-with-col" .S cr
            dup -1 = if 
                drop 1+ 
                matrix matrix-count-columns c@ 1- over <
                \ ." Not found yet" .S cr
                if
                    \ we ran out of matrix
                    256 true
                else
                    false
                endif
            else
                \ ." Found" .S cr
                true
            endif
        until
        \ ." Candidate row" .S cr

        ( column row-with-col )

        matrix matrix-count-columns c@ 1- over 
        \ ." Checking out of columns?" .S cr
        < if
            \ we ran out of columns and went into the answer column
            \ we must be done
            \ matrix print-matrix
            \ ." Ran out of data?" .S cr
            2drop unloop exit
        endif

        ( column row-with-col )

        dup i <> if
            \ ." Swapping" .S cr
            matrix i rot swap-matrix-rows
            \ matrix print-matrix
        else
            drop
        endif

        \ row i now has nonzero in column
        \ eliminate below it
        matrix matrix-count-rows c@ i 1+ ?do
            \ ." Eliminating j i " j . i . .S cr
            \ subtract row j from row i
            ( column )
            dup matrix j rot matrix-at
            ( column mjc )
            over matrix i rot matrix-at
            ?dup 0<> if
                ( column mjc mic )
                \ j is 0 0 mjc . . .
                \ i is 0 0 mic . . .
                \ want 0 0 mic-mjc*mic/mjc . . .
                \ avoiding fractions,
                \      0 0 lcm-lcm
                \ =    0 0 mic*(lcm/mic) - mjc*lcm/mjc
                \ i.e. scale i by (lcm/mic), add row j * -lcm/mjc
                2dup lcm
                ( column mjc mic lcm )
                swap over swap /
                ( column mjc lcm lcm/mic )
                matrix i rot scale-matrix-row
                \ matrix print-matrix 
                ( column mjc lcm )
                swap /
                ( column mjc/lcm )
                matrix j rot i swap negate add-matrix-rows

                 \ if row i now starts negative, negate it
                 matrix i matrix-row-starts-negative if
                    matrix i -1 scale-matrix-row
                 endif
            else
                ( column mjc )
                drop
            endif
        LOOP

        \ ." After step " i . .S cr
        \ matrix print-matrix cr

        1+
    LOOP
    drop
;

: solve-power { machine -- answer }
    \ This isn't search at all, this is algebra...
    \ (0,2) (1,2) (1)  {a,b,c}
    \ is actually a discrete math problem
    \ k0 = a, k1 + k2 = b, k0 + k1 = c, k0,k1,k2 >= 0, minimise k0+k1+k2
    \ i.e.
    \ [ 1  0  0  ] [k0] = [a]
    \ [ 0  1  1  ] [k1] = [b]
    \ [ 1  1  0  ] [k2] = [c]
    \ where the buttons form the columns of the matrix
    \ and the goals form the outcome
    \ and we need to determine the values of k =

    \ However
    \ It's actually an integer linear programming thing, which is known to be NP,
    \ and the solutions are kind of a pain
    \ so, what if we just do search anyway i guess?
    \ ok fine, after doing some gaussian elimination =

    machine machine>matrix
    ." Constructed matrix" cr
    dup print-matrix
    dup gaussian-eliminate
    ." Eliminated matrix" .S cr
    print-matrix

    \ Search state is #lights * 9 bits, max of 90 bits; can fit in a double cell = 2 x 8 bytes = 2 x 64 bits = 128 bits
    \ Actually using double cell is a bit awkward, so instead we just use 2 cells, putting 45 bits in each
    \ and we can store the distance travelled in the top 16 bits of one of the cells ==

    
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