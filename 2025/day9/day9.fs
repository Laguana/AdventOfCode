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

: print-rect
    idx>point dup point-x @ . ',' emit point-y @ .
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

: between ( p a b -- bool )
    2dup min -rot max { p low high }
    p low > p high < and
; \ =
: between-inclusive ( p a b -- bool )
    2dup min 1- -rot max 1+ between
;

: interior? ( a b -- bool )
    \ A rectangle formed by 2 points is interior
    \ if no other points are strictly inside it
    \ and no other sequential pairs cross over it
    \ .... unless there's shenannigans with adjacent lines. lets try not
    \ =
    idx>point dup point-x @ swap point-y @
    ( a bx by )
    rot
    idx>point dup point-x @ swap point-y @
    { bx by ax ay }

    day-9-point-count @ 1- idx>point dup
    point-x @ swap point-y @
    ( x y )
    2dup 
    by ay between swap bx ax between and if \ =
        \ point n-1 is strictly inside this 
        2drop 
        false exit
    endif

    ( x y )
    day-9-point-count @ 0 ?do
        \ ." Interior main loop" i .S drop cr
        i idx>point dup
        point-x @ swap point-y @
        ( px py x y )
        \ ." Interior coords" .S cr
        2dup by ay between swap bx ax between 
        and if \ =
            \ point i is strictly inside this
            2drop 2drop unloop
            false exit
        endif

        \ check the i-1 -> i line
        ( px py x y )
        rot
        ( px x y py)
         2dup = if
            \ ." Horizontal" .S cr
            \ y = py, horizontal line
            \ intersects if y is between ay by
            \ and also if px->x contains ax or bx, including the ends =
            ay by between if
                ax 2over 
                2dup bx -rot
                ( px x y ax px x bx px x)
                between-inclusive
                ( px x y ax px x bx-between )
                >r
                between-inclusive r> or if
                    \ either ax or bx is strictly inside
                    ( px x y )
                    2drop drop unloop false exit
                else
                    \ not intersecting
                    rot drop
                endif
            else
                rot drop
                ( x y )
            endif
        else
            \ ." Vertical" .S cr
            \ y != py, vertical line
            \ intersects if x is between ax bx
            \ and also if py->y contains ay or by =
            ( px x y py )
            2swap swap
            ( y py x px )
            ax bx between if
                ( y py x )
                ay 2over 
                2dup by -rot
                ( y py x ay y py by y py )
                between-inclusive
                ( y py x ay y py by-between )
                >r
                between-inclusive
                r> or if
                    \ either ay or by is strictly inside
                    \ ." hit vertical" .S cr
                    ( y py x )
                    2drop drop unloop false exit
                else
                    \ not intersecting
                    -rot drop
                endif
            else
                -rot drop
                ( x y )
            endif
        endif
    loop
    2drop
    true
;

: max-rect-size-2
    0
    day-9-point-count @ 1- 0 ?do
        day-9-point-count @ i 1+ ?do
            i j rectangle-size
            ( max next )
            2dup < if
                \ this rectangle is worth considering
                \ ." Considering max" i print-rect j print-rect .S cr cr key drop
                i j interior? if
                    \ ." Checks out"
                    \ this is valid, take it
                    nip
                else
                    \ it is not valid
                    drop
                endif
            else
                drop
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
  r@ day-9-part-2 . \ assert( 24 = )
  r>
  close-file throw
;

( Expect to get 1465767840 )
: do-day-9-part-2
  s" day9.in" r/o open-file throw ( -- fd )
  >r
  r@ day-9-part-2 .
  r>
  close-file throw
;