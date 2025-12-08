: is-digit? ( c -- digit? )
    dup '0' >= swap '9' <= and \ =
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