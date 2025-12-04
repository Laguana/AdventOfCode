20480 CONSTANT day-4-buf-size
CREATE day-4-buf day-4-buf-size allot
VARIABLE day-4-width
VARIABLE day-4-height

: set-width ( -- ) \ sets day-4-width
    day-4-buf
    begin
        ( ptr )
        dup c@ 
        10 <>
    while
        1+
    repeat
    \ ptr points at the first newline
    day-4-buf -
    day-4-width !
;

: set-height ( size -- ) \ sets day-4-height
    day-4-width @ 1+ /
    day-4-height !
;

: day-4-initialize ( fd -- ) \ initializes variables
    day-4-buf day-4-buf-size rot read-file throw 
    ( bytes-read )
    set-width
    set-height
    \ stack is empty, but variables are initialized
;

: cell-full? { x y -- full? }
    y 0<
    x 0<
    y day-4-height @ >=
    x day-4-width @ >=
    or or or if 
        false exit
    endif
    y day-4-width @ 1+ * x +
    day-4-buf + c@ '@' =
;

: clear-cell ( x y -- )
    day-4-width @ 1+ * +
    day-4-buf + 'x' swap c!
;

: cell-neighbors { x y -- #neighbors }
    -1
    2 -1 do
        2 -1 do
            x i + y j + cell-full? if
                1+
            endif
        loop
    loop
;

: day-4-part-1 ( fd -- answer )
    day-4-initialize

    0
    day-4-height @ 0 do
        day-4-width @ 0 do
            \ i = x, j = y
             i j cell-full? if
                i j cell-neighbors 4 < if
                    1+
                endif
             endif
        LOOP
    LOOP
;

: day-4-part-2 ( fd -- answer )
    day-4-initialize
    
    0
    begin
        dup
        day-4-height @ 0 do
            day-4-width @ 0 do
                \ i = x, j = y
                i j cell-full? if
                    i j cell-neighbors 4 < if
                        i j clear-cell
                        1+
                    endif
                endif
            LOOP
        LOOP
        ( previous current)
        swap over =
        ( current eq? )
    until
;

: test-day-4-part-1
  s" day4.example" r/o open-file throw ( -- fd )
  >r
  r@ day-4-part-1  assert( 13 = )
  r>
  close-file throw
;

( Expect to get 1474 )
: do-day-4-part-1
  s" day4.in" r/o open-file throw ( -- fd )
  >r
  r@ day-4-part-1 .
  r>
  close-file throw
;

: test-day-4-part-2
  s" day4.example" r/o open-file throw ( -- fd )
  >r
  r@ day-4-part-2  assert( 43 = )
  r>
  close-file throw
;

( Expect to get 8910 )
: do-day-4-part-2
  s" day4.in" r/o open-file throw ( -- fd )
  >r
  r@ day-4-part-2 .
  r>
  close-file throw
;