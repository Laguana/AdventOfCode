: is-digit? ( char -- bool )
    dup '0' >=
    swap '9' <=
    and
;

\ = this is just to fix syntax hilighting

: read-number ( fd -- number next-char )
    >r
    0
    begin
        r@ file-eof?
        if
            r> drop
            -1
            exit
        endif
        r@ key-file
        dup is-digit?
    while
        '0' -
        swap 10 * +
    repeat
    r> drop
;

: log-10 ( n -- log10 )
    assert( dup 0> )
    >r
    0 10

    begin
        dup 1- r@ <
    while
        10 *
        swap 1+
        swap
    repeat
    drop
    r> drop
;

: digit-length ( n -- length )
    log-10 1+
;

: pow10 ( d -- 10^d )
    1 swap 0 ?do 10 * loop
;

: remove-digits ( n d -- n )
    pow10 /
;

: extend-digits ( n d -- n )
    pow10 *
;


\ Only even length numbers can be interesting here

: invalid-in-range ( start stop -- invalid-sum )
    over digit-length 
    over digit-length
    \ .S cr cr
    ( start stop #start #stop )
    2dup =
    if
        2 mod 1 =
        if
            \ the range is entirely within an odd digit number, we can ignore it
            drop drop drop 0
            \ S" Early exit from odd range" type cr
            exit
        endif
    else
        drop
    endif
    ( start stop #start)
    swap >r
    0 -rot
    ( acc start #start )
    dup 2 mod 1 =
    if 
        \ the start is on an odd digit count, bump to next power of 10
        \ but also only keep the first half
        nip 2 / dup pow10
    else 
        \ we start on an even count, but need to find the starting point that still is within the range
        2 / over over remove-digits
        ( acc start #start/2 start>>n )
        over over swap extend-digits
        ( acc start #start/2 start>>n start>>n<<n)
        over +
        ( acc start #start/2 start>>n stasta )
        3 pick over > if
            ( acc start #start/2 start>>n stasta )
            \ S" maybe dodgy?" type .S cr
            drop 1+ 
            over over swap extend-digits over +
        endif
        ( acc start #start/2 start>>n?+1 stasta)
        dup
        r@ > if
            \ We started at 123456, but 123123 is too big to fit, bail
            \ S" Exiting, too big to fit" type .S cr
            drop drop drop drop
            r> drop
            exit
        endif
        ( acc start #start/2 start>>n stasta )
        drop rot drop
    endif
    ( acc #i i )
    nip
    ( acc i )
    \ S" beginning main loop" type .S cr
    begin
        dup dup digit-length 
        extend-digits
        over +
        dup
        r@ <=
        ( acc i ii ii<=end)
        \ .S cr
    while
        \ S" found one" type .S cr
        rot +
        swap 
        1+
    repeat

    drop drop
    r> drop
;

: day-2-part-1 ( fd -- result )
  >r
  0
  begin
    ( accumulator )
    r@ read-number
    assert( '-' = )
    r@ read-number
    ( accumulator start stop next-char)
    ',' <>
    \ .S cr cr
    >r
    invalid-in-range +
    r>
    \ .S cr cr cr
  until
  r> drop
;

: test-day-2-part-1
  s" day2.example" r/o open-file throw ( -- fd )
  >r
  r@ day-2-part-1  . \ assert( 1227775554 = )
  r>
  close-file throw
;

( Expect to get 54234399924 )
: do-day-2-part-1
  s" day2.in" r/o open-file throw ( -- fd )
  >r
  r@ day-2-part-1 .
  r>
  close-file throw
;