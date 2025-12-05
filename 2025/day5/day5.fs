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

: day-5-part-2 ( fd -- answer )
    parse-ranges
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
  r@ day-5-part-2 . \ assert( 14 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-5-part-2
  s" day5.in" r/o open-file throw ( -- fd )
  >r
  r@ day-5-part-2 .
  r>
  close-file throw
;