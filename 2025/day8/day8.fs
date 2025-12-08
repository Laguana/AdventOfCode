require ../common/common.fs

20 CONSTANT day-8-buf-len
CREATE day-8-buf day-8-buf-len allot
VARIABLE day-8-junction-boxes
VARIABLE day-8-junction-count
VARIABLE day-8-circuit-count
0 day-8-circuit-count !
VARIABLE day-8-pair-connected

struct
    cell% field junction-x
    cell% field junction-y
    cell% field junction-z
    cell% field junction-circuit
    cell% field junction-next-index
end-struct junction-box%

: print-junction ( ptr -- )
    '(' emit dup junction-x @ . ',' emit dup junction-y @ . ',' emit dup junction-z @ . ':' emit junction-circuit @ . ')' emit
;

: junction-sq-distance { p1 p2 -- dist }
    p1 junction-x @ p2 junction-x @ - dup *
    p1 junction-y @ p2 junction-y @ - dup * +
    p1 junction-z @ p2 junction-z @ - dup * +
;

: junction-ptr ( idx -- ptr )
    junction-box% %size *
    day-8-junction-boxes @ +
;

: parse-input ( fd -- ) \ initialize variables
    >r
    HERE day-8-junction-boxes !
    0
    begin
        \ S" reading input" type .S cr
        day-8-buf day-8-buf-len r@ read-line throw
    while
        ( #lines #read)
        day-8-buf + -1 swap c!

        \ read the 3 numbers, store in a junction-box shape
        day-8-buf get-number , 1+
        get-number , 1+
        get-number , drop
        0 ,
        dup , \ it points to its own index
        1+
    repeat
    drop
    day-8-junction-count !

    r> drop
;

: join-circuits { start circuit -- last }
    start
    dup junction-ptr junction-circuit circuit swap !
    begin
        ( i )
        dup junction-ptr junction-next-index @
        ( i ni )
        dup junction-ptr dup junction-circuit @ circuit <>
        ( i ni pni pni-circuit=circuit? )
    while
        rot drop
        ( ni pni )
        junction-circuit circuit swap !
    repeat
    2drop
;

: connect-junctions { a b -- }
    a junction-ptr junction-circuit @ 0= if
        b junction-ptr junction-circuit @ 0= if
            \ fresh circuit made
            1 day-8-circuit-count +!
            day-8-circuit-count @ dup
            a junction-ptr junction-circuit !
            b junction-ptr junction-circuit !
            b a junction-ptr junction-next-index !
            a b junction-ptr junction-next-index !
        else
            \ a joins b's circuit
            b junction-ptr junction-circuit @ a junction-ptr junction-circuit !
            b junction-ptr junction-next-index
            dup @ a junction-ptr junction-next-index !
            a swap !
        endif
    else
        b junction-ptr junction-circuit @ 0= if
            \ b joins a's circuit
            a junction-ptr junction-circuit @ b junction-ptr junction-circuit !
            a junction-ptr junction-next-index
            dup @ b junction-ptr junction-next-index !
            b swap !
        else
            \ merge circuits together
            \ arbitrarily pick b as the one to merge into a
            b a junction-ptr junction-circuit @ join-circuits
            ( last-in-b )
            junction-ptr 
            assert( dup 
                \ dup print-junction cr 
                junction-next-index 
                \ .S cr 
                @ b = )
            junction-next-index a junction-ptr junction-next-index @ swap !
            b a junction-ptr junction-next-index !
        endif
    endif
;

: assign-circuits ( n -- )
    day-8-junction-count @ dup * dup allocate throw 
    { pair-table-size pair-table }
    pair-table pair-table-size erase
    0 do
        \ assign n shortest connections
        -1 -1 -1 
        ( idx1 idx2 distance )
        day-8-junction-count @ 1- 0 do
            \ find next shortest link
            \ S" Outer loop" type .S cr
            day-8-junction-count @ i 1+ do
                \ j is start, i is inner, see if |i-j| is shorter than current min distance
                \ check if they are already connected?
                j day-8-junction-count @ * i + pair-table + c@ 0 <> if
                    \ the junctions were already connected
                else
                    \ junctions are not connected
                    i junction-ptr 
                    j junction-ptr
                    ( idx1 idx2 distance i-ptr j-ptr )
                    junction-sq-distance 2dup u<=
                    \ S" Better?" type i j .S 2drop cr 
                    \ i junction-ptr print-junction
                    \ j junction-ptr print-junction cr
                    if
                        drop
                    else
                        nip
                        ( idx1 idx2 newdistance )
                        -rot 2drop i j rot
                    endif
                endif
            loop
        loop
        drop
        \ i1 i2 are next smallest to be connected
         S" next smallest found" i . type .S cr
         2dup junction-ptr print-junction cr junction-ptr print-junction cr
        \ 2dup junction-ptr swap junction-ptr junction-sq-distance . cr

        2dup day-8-junction-count @ * + pair-table + -1 swap c!

        2dup junction-ptr junction-circuit @ swap junction-ptr junction-circuit @ dup 0= -rot <> or if
            connect-junctions
        else
            2drop
        endif
    loop
    pair-table free throw
;

: sum-biggest-3 ( -- answer )
    day-8-circuit-count @ cells dup allocate throw { sizes-table-size sizes-table }
    sizes-table sizes-table-size erase

    S" Adding sizes" type .S cr
    day-8-junction-count @ 0 do
        i junction-ptr
        \ dup print-junction cr
        junction-circuit @
        ( i-circuit )
         dup 0 <> if
            1- cells sizes-table + 1 swap +!
        else
            drop
        endif
    loop

    S" Finding max3" type .S cr

    1
    3 0 do
        ( acc )
        -1 0
        ( acc idx max )
        day-8-circuit-count @ 0 do
            sizes-table i cells + @
            2dup < if
                ( acc idx oldmax newmax)
                nip nip i swap
            else
                drop
            endif
        loop
        S" Max iteration" type .S cr
        rot * swap cells sizes-table + 0 swap !
    loop

    sizes-table free throw
;


: day-8-part-1 ( fd n -- answer )
    swap parse-input
    assign-circuits
    sum-biggest-3
;

: day-8-part-2 ( fd -- answer )
    drop 0
;


: test-day-8-part-1
  s" day8.example" r/o open-file throw ( -- fd )
  >r
  r@ 10 day-8-part-1 assert( 40 = )
  r>
  close-file throw
;

( Expect to get 330786 )
: do-day-8-part-1
  s" day8.in" r/o open-file throw ( -- fd )
  >r
  r@ 1000 day-8-part-1 .
  r>
  close-file throw
;

: test-day-8-part-2
  s" day8.example" r/o open-file throw ( -- fd )
  >r
  r@ day-8-part-2  assert( 0 = )
  r>
  close-file throw
;

( Expect to get ? )
: do-day-8-part-2
  s" day8.in" r/o open-file throw ( -- fd )
  >r
  r@ day-8-part-2 .
  r>
  close-file throw
;