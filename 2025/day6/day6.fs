20000 CONSTANT day-6-buf-size
CREATE day-6-buf day-6-buf-size allot
VARIABLE day-6-num-columns
VARIABLE day-6-data
VARIABLE day-6-num-rows

: is-digit? ( c -- digit? )
    dup '0' >= swap '9' <= and \ =
;

: find-next-number ( ptr -- ptr eol? )
    begin
        dup c@ 32 =
    while
        1+
    repeat
    dup c@ 10 =
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

: read-one-row ( ptr -- ptr )
    day-6-num-columns @ 0 DO
        find-next-number assert( invert )
        get-number ,
    LOOP
;

: parse-input ( fd -- ptr) \ initializes variables
    day-6-buf day-6-buf-size rot read-file throw
    day-6-buf + -1 swap c!

    HERE day-6-data !
    0 day-6-buf
    \ S" before loop" type .S cr
    begin
        ( count ptr )
        find-next-number
        ( count ptr eol? )
        invert
        \ over c@ . S" In loop" type .S cr key drop
    while
        get-number ,
        swap 1+ swap
    repeat
    \ we read the first line, store the length
    swap day-6-num-columns !
    ( ptr )
    1+
    1 swap
    ( #rows ptr )
    begin
        find-next-number if 1+ find-next-number drop endif
        dup c@ is-digit?
        over c@ . S" Rows after 1" type .S cr
    while
        swap 1+ swap
        read-one-row
    repeat
    swap day-6-num-rows !
    \ Now we've read all the numbers, we know how many there are, and
    \ we're pointing exactly at the first operation =
;

: add-column ( idx -- ans )
    0 swap cells day-6-data @ +
    day-6-num-rows @ 0 DO
        ( acc ptr )
        dup @ rot + swap
        day-6-num-columns @ cells +
    LOOP
    drop
;

: mul-column ( idx -- ans )
    1 swap cells day-6-data @ +
    day-6-num-rows @ 0 DO
        ( acc ptr )
        dup @ rot 
        S" mul" type .S cr 
        * swap
        day-6-num-columns @ cells +
    LOOP
    drop
;

: day-6-part-1 ( fd -- answer )
    parse-input

    0 swap
    day-6-num-columns @ 0 DO
        ( acc ptr )
        find-next-number drop
        ( acc ptr )
        dup c@ '+' = if
            i add-column
        else
            i mul-column
        endif
        S" column " type i . .S cr
        rot + swap
        1+ 
    LOOP
    drop
;

: day-6-part-2 ( fd -- answer )
    drop 0
;

: test-day-6-part-1
  s" day6.example" r/o open-file throw ( -- fd )
  >r
  r@ day-6-part-1 assert( 4277556 = )
  r>
  close-file throw
;

( Expect to get 532200471868 )
: do-day-6-part-1
  s" day6.in" r/o open-file throw ( -- fd )
  >r
  r@ day-6-part-1 .
  r>
  close-file throw
;

: test-day-6-part-2
  s" day6.example" r/o open-file throw ( -- fd )
  >r
  r@ day-6-part-2 . \ assert( 0 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-6-part-2
  s" day6.in" r/o open-file throw ( -- fd )
  >r
  r@ day-6-part-2 .
  r>
  close-file throw
;