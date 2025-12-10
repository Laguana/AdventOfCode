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