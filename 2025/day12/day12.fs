require ../common/common.fs

CREATE day-12-buffer 25 allot
CREATE day-12-tile-fullness 6 cells allot

: count-full-in-buf ( -- full )
    day-12-buffer
    dup c@ '#' = negate
    swap 1+ dup c@ '#' = negate
    swap 1+ c@ '#' = negate + +
;

: parse-input { fd -- fd }
    6 0 DO
        day-12-buffer 25 fd read-line throw
        assert( )
        drop

        day-12-buffer 25 fd read-line throw
        assert( )
        assert( 3 = )
        count-full-in-buf

        day-12-buffer 25 fd read-line throw
        assert( )
        assert( 3 = )
        count-full-in-buf +
        
        day-12-buffer 25 fd read-line throw
        assert( )
        assert( 3 = )
        count-full-in-buf +

        day-12-tile-fullness i cells + !

        day-12-buffer 25 fd read-line throw
        assert( )
        drop
    LOOP
    fd
;

: day-12-part-1 ( fd -- answer)
    parse-input
    >r
    0
    begin
        day-12-buffer 25 r@ read-line throw nip
    while
        ." Starting "
        day-12-buffer 25 type cr .S cr

        day-12-buffer get-number
        ( acc ptr n1 )
        swap 1+ get-number
        ( acc n1 ptr n2)
        \ ." area? " .S cr
        rot *
        ( acc ptr area )
        \ .S cr
        swap
        2 +
        6 0 DO
            get-number
            ( acc area ptr ni )
            day-12-tile-fullness i cells + @ *
            \ ." tile fullness " .S cr
            ( acc area ptr tile-area )
            negate rot +
            ( acc ptr area )
            swap 1+
            ( acc area ptr)
        LOOP
        drop
        \ ." Area after " .S cr
        0> if 
            \ If summing up all the full spaces is more than
            \ the available spaces, it can't possibly fit
            \ If it is not more, then in theory it might still
            \ not fit, as demonstrated by the example...
            \ but the given inputs are very unconstrained.
            1+
        endif
    repeat
    r>
    drop
;


( Expect to get 583 )
: do-day-12-part-1
  s" day12.in" r/o open-file throw ( -- fd )
  >r
  r@ day-12-part-1 .
  r>
  close-file throw
;