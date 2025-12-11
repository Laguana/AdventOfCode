: is-digit? ( c -- digit? )
    dup '0' >= swap '9' <= and \ =
;

: skip-spaces ( ptr -- ptr )
    begin
        dup c@ 32 =
    while
        1+
    repeat
;

: find-next-number-or-eol ( ptr -- ptr eol? )
    begin
        dup c@ dup is-digit? swap 10 = or
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

: max ( a b -- c )
    2dup > if drop else nip endif
;
: min ( a b -- c )
    2dup > if nip else drop endif
;
: umin ( a b -- c )
    2dup u> if nip else drop endif
;

: count-bits-set ( n -- #bits )
    0 swap
    begin
        dup 1 and rot + swap 2/
        ?dup 0=
    until
;

: bit-set? ( n i -- set? )
    1 swap lshift and 0<>
;

: qsort-partition { smaller? exchange low high -- pivot}
    \ ." partition" .S cr key drop
    low
    high low ?do
        ( pivot_index ) \ pivot_index is the earliest index bigger than the pivot at high
        i high smaller? execute 
        if
            \ ." partition smaller" i high .S 2drop cr
            \ i is smaller than the pivot value
            \ swap i into the pivot index and increase pivot index =
            dup i 2dup <> if exchange execute else 2drop endif
            1+
        endif
    LOOP
    \ ." done partition loop" .S cr key drop
    dup high exchange execute 
;
: qsort ( smaller? exchange low high -- ) \ low and high are inclusive indices =
    \ ." qsort" .S cr
    2swap { smaller? exchange }
    ( low high )
    begin
        \ ." qsort start " .S cr
        over 2dup
        ( low high low high low )
        0 >= -rot < and  \ =
    while
        ( low high )
        2dup smaller? exchange 2swap qsort-partition
        \ ." qsort found partition" .S cr
        ( low high partition )
        >r 
        ( low high R: pivot )
        2dup r@ - abs swap r@ - abs > 
        ( low high high-pivot>low-pivot) if
            \ high-pivor > low-pivot, recurse low
            smaller? rot exchange swap r@ 1- recurse
            ( high )
            r> 1+ swap
            ( pivot+1 high )
        else
            \ low-pivot > high-pivot, recurse high
            smaller? exchange rot r@ 1+ swap recurse
            ( low )
            r> 1-
            ( low pivot-1 )
        endif
    repeat
    \ ." qsort end" .S cr
    2drop
;

struct
    cell% field associative-map-#buckets
    double% drop 0 associative-map-buckets
end-struct associative-map%

\ associative map from int to cell, excluding 0
\ removing not supported
: create-associative-map ( #buckets -- map )
    dup associative-map% rot 2 * cells + %alloc
    ( #buckets map )
    2dup associative-map-#buckets !
    swap 2 * cells over erase
;

: associative-map-lookup { map entry -- result? found? }
    map associative-map-#buckets @ entry over mod
    ( #buckets initial )
    swap { #buckets }
    #buckets 0 do
        ( candidate-index )
        map associative-map-buckets over 2* cells +
        dup @
        ( candidate-index bucket-ptr bucket-key)
        dup 0= if nip nip unloop exit endif
        entry = if
            1+ @ nip true unloop exit
        endif
        drop
        1+ #buckets mod
    LOOP
    assert( ." entry not found and never hit an empty cell" false )
;

: associative-map-set { map entry value -- previous }
    map associative-map-#buckets @ entry over mod
    ( #buckets initial )
    swap { #buckets }

;