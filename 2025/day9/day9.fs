require ../common/common.fs

20 CONSTANT day-9-buf-len
CREATE day-9-buf day-9-buf-len allot
VARIABLE day-9-point-list
variable day-9-point-count

struct
    cell% field point-x
    cell% field point-y
end-struct point%

: parse-input ( fd -- ) \ populates variables
    >r
    HERE day-9-point-list !
    0
    begin
        \ ." reading input" .S cr
        day-9-buf day-9-buf-len r@ read-line throw
    while
        ( #lines #read)
        day-9-buf + -1 swap c!

        \ read the 2 numbers, store in a point shape
        day-9-buf get-number , 1+
        get-number , drop
        1+
    repeat
    drop
    day-9-point-count !

    r> drop
;

: idx>point
    point% %size * day-9-point-list @ +
;

: rectangle-size ( i j -- size )
    idx>point swap idx>point { jp ip }
    jp point-x @ ip point-x @ - abs 1+
    jp point-y @ ip point-y @ - abs 1+
    \ ." rectangle-size" .S cr
    *
;

: max-rect-size 
    0
    day-9-point-count @ 1- 0 ?do
        day-9-point-count @ i 1+ ?do
            i j rectangle-size 
            max
        loop
    loop
;

: interior?
    2drop 0
;

: max-rect-size-2
    0
    day-9-point-count @ 1- 0 ?do
        day-9-point-count @ i 1+ ?do
            i j interior? if 
                i j rectangle-size 
                max
            endif
        loop
    loop
;

: day-9-part-1 ( fd -- answer)
    parse-input
    max-rect-size
;

: day-9-part-2 ( fd -- answer)
    parse-input
    max-rect-size-2
;

: test-day-9-part-1
  s" day9.example" r/o open-file throw ( -- fd )
  >r
  r@ day-9-part-1 assert( 50 = )
  r>
  close-file throw
;

( Expect to get 4752484112 )
: do-day-9-part-1
  s" day9.in" r/o open-file throw ( -- fd )
  >r
  r@ day-9-part-1 .
  r>
  close-file throw
;

: test-day-9-part-2
  s" day9.example" r/o open-file throw ( -- fd )
  >r
  r@ day-9-part-2  assert( 0 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-9-part-2
  s" day9.in" r/o open-file throw ( -- fd )
  >r
  r@ day-9-part-2 .
  r>
  close-file throw
;