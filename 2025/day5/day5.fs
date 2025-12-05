22000 CONSTANT day-5-buf-size
CREATE day-5-buf day-5-buf-size allot
VARIABLE day-5-num-ranges
VARIABLE day-5-ranges

: is-digit? ( c -- digit? )
    dup '0' >= swap '9' <= and \ =
;

: get-number ( ptr -- ptr n ) \ returned pointer points past the number
    0 swap begin
        ( acc ptr )
        dup c@ dup is-digit?
    while
        ( acc ptr @ptr )
        '0' - rot 10 * + swap
        1+
    repeat
    drop swap
;

: count-ranges ( -- #ranges )
    1 day-5-buf
    begin
        ( #lines pointer )
         dup c@ 10 =
        if
            dup 1+ c@ 10 <>
            if
                swap 1+
                swap 1+
            else
                drop exit
            endif
        else
            1+
        endif
    again
;
: read-ranges ( -- ptr )
    HERE day-5-ranges !
    day-5-buf
    day-5-num-ranges @ 0 do
        ( ptr )
        get-number , 1+
        get-number , 1+
    loop

;

: parse-ranges ( fd -- ptr )
    day-5-buf day-5-buf-size rot read-file throw
    ( nwritten )
    day-5-buf + -1 swap c!

    count-ranges day-5-num-ranges !
    read-ranges
;

: is-in-range? { n -- inrange? }
    day-5-ranges @
    day-5-num-ranges @ 0 
    DO
        ( rangeptr )
        dup 2@ 
        ( rangeptr low high)
        n > swap n < or if
            2 cells +
        else
            drop unloop true exit
        endif
    loop
    drop
    false
;

: day-5-part-1 ( fd -- answer )
    parse-ranges
    ( ptr )
    0 swap
    begin
        ( acc ptr)
        dup c@ 255 <>
    while
        1+
        get-number 
        \ S" Checking number" type .S cr key drop 
        is-in-range? if
            ( acc ptr )
            swap 1+ swap
        endif
    repeat
    drop
;

: sort-ranges ( -- ) \ in-place insertion sort is fine right?
    \ first element is already inserted, insert the rest
    day-5-num-ranges @ 1 DO
        \ we have inserted prior to index i already, now we are inserting i
        i
        day-5-ranges @ i 2 * cells + >r
        r@ 2@
        \ S" Outer loop" type .S cr
        ( i h l )
        rot 0 DO
            \ if it is smaller, swap the one we have with the slot we're looking at
            \ i is now the inner loop, j the outer
            day-5-ranges @ i 2 * cells + >r
            r@ @
            ( h l x)
            \ S" Inner loop" type .S cr key drop
            over > if
                \ we do want to swap
                r@ 2@
                ( h l xh xl)
                2swap r@ 2!
            else
                r@ @ over = if
                    ( h l ) \ we might want to swap if the high is lower
                    r@ 1+ @
                    ( h l y )
                    2 pick > if
                        ( h l ) \ we do want to swap
                        r@ 2@
                        ( h l xh xl )
                        2swap r@ 2!
                    endif
                endif
            endif
            r> drop
        LOOP
        ( h l )
        r> 2!
    LOOP
;

: day-5-part-2 ( fd -- answer )
    parse-ranges drop
    sort-ranges

    0 0
    day-5-num-ranges @ 0 DO
        ( acc previous-high )
        \ S" Start loop" type .S cr
        day-5-ranges @ i 2 * cells + 2@
        ( acc previous-high high low )
        rot
        ( acc high low previous-high )
        2dup <= if
            \ this range intersects
            \ S" Range intersects" type .S cr
            ( acc high low previous-high )
            nip
            1+
            ( acc high new-low)
        else
            drop
        endif
        ( acc high low )
        2dup >= if
            \ after we adjusted the range, it is still nonempty
            over swap - 1+
            ( acc high high-low )
            \ S" Found a range " type .S cr
            rot + swap
        else
            drop
        endif
    LOOP
    drop
;

: test-day-5-part-1
  s" day5.example" r/o open-file throw ( -- fd )
  >r
  r@ day-5-part-1  assert( 3 = )
  r>
  close-file throw
;

( Expect to get 635 )
: do-day-5-part-1
  s" day5.in" r/o open-file throw ( -- fd )
  >r
  r@ day-5-part-1 .
  r>
  close-file throw
;

: test-day-5-part-2
  s" day5.example" r/o open-file throw ( -- fd )
  >r
  r@ day-5-part-2  assert( 14 = )
  r>
  close-file throw
;

( Expect to get 369761800782619 )
: do-day-5-part-2
  s" day5.in" r/o open-file throw ( -- fd )
  >r
  r@ day-5-part-2 .
  r>
  close-file throw
;