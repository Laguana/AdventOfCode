require ../common/common.fs

20 CONSTANT day-8-buf-len
CREATE day-8-buf day-8-buf-len allot
VARIABLE day-8-junction-boxes
VARIABLE day-8-junction-count
VARIABLE day-8-circuit-count
0 day-8-circuit-count !
VARIABLE day-8-distances

struct
    cell% field junction-x
    cell% field junction-y
    cell% field junction-z
    cell% field junction-circuit
    cell% field junction-next-index
end-struct junction-box%

struct
    cell% field distance-entry-i
    cell% field distance-entry-j
    cell% field distance-entry-distance
end-struct distance-entry%

: print-junction ( ptr -- )
    '(' emit dup junction-x @ . ',' emit dup junction-y @ . ',' emit dup junction-z @ . ':' emit junction-circuit @ . ')' emit
;

: junction-sq-distance { p1 p2 -- dist }
    p1 junction-x @ p2 junction-x @ - dup *
    p1 junction-y @ p2 junction-y @ - dup * +
    p1 junction-z @ p2 junction-z @ - dup * +
;

: junction-ptr ( idx -- ptr )
    [ junction-box% %size ]L *
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

: populate-distances ( -- )
    day-8-junction-count @ dup 1- * 2 / distance-entry% %size * dup 
    allocate throw
    day-8-distances !
    day-8-distances @ swap erase

    \ We now have n*(n-1)/2 distance-entry fields to populate

    day-8-distances @
    ( ptr )
    day-8-junction-count @ 1- 0 do
        day-8-junction-count @ i 1+ do
            dup distance-entry-i i swap !
            dup distance-entry-j j swap !
            dup distance-entry-distance 
            i junction-ptr 
            j junction-ptr
            junction-sq-distance
            swap !
            [ distance-entry% %size ]L +
        loop
    loop
    \ ." populated distances" .S cr key drop
    drop
;

: idx->distance ( i -- ptr )
    [ distance-entry% %size ]L * day-8-distances @ +
;

: swap-distances { l r -- }
    ." swap-distances" l r .S 2drop cr key drop
    l idx->distance
    dup dup dup
    distance-entry-i @ rot
    distance-entry-j @ rot
    distance-entry-distance @
    ( lp li lj ldistance )
    >r >r >r
    ( lp R: ldistance lj li )
    r idx->distance dup >r
    swap [ distance-entry% %size ]L move
    \ r is copied to l, now to put l where r was
    r> dup dup
    distance-entry-i r> swap !
    distance-entry-j r> swap !
    distance-entry-distance r> swap !
;

: distance-smaller ( i j -- i<=j )
    ." Distance smaller" .S cr
    idx->distance distance-entry-distance @
    swap
    idx->distance distance-entry-distance @
    ( jdist idist )
    ." -Distance smaller" .S cr
    >=
;

: sort-distances ( -- )
    ['] distance-smaller ['] swap-distances 0 
    day-8-junction-count @ dup 1- * 2 / 1- qsort
;

: assign-circuits ( n -- )
    0 do
        \ assign n shortest connections
        i idx->distance
        dup distance-entry-i @
        swap distance-entry-j @

        ( i j )
        ." assign-circuits pair " .S cr
         2dup junction-ptr print-junction cr junction-ptr print-junction cr
         2dup junction-ptr swap junction-ptr junction-sq-distance . cr

        2dup junction-ptr junction-circuit @ swap junction-ptr junction-circuit @ dup 0= -rot <> or if
            connect-junctions
        else
            2drop
        endif
    loop
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
    populate-distances
    sort-distances
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