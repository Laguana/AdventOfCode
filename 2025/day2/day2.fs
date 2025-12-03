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
    1 swap 0 +do 10 * loop
;

: remove-digits ( n d -- n )
    assert( dup 0 >= )
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

: repeat-n-times { s n -- sss }
    s digit-length 
    pow10 
    0 n 0 ?DO
        over * s +
    LOOP
    nip
;

: repeats-n-times-sum { start stop n -- sum }
    \ Find the sum of numbers betwen start and stop inclusive that consist of
    \ some sequence repeating exactly n times

    stop digit-length n < if
        \ We can't repeat n times, it doesn't fit in the end
        \ S" Early exit, stop is too short" type .S cr
        0
        exit
    endif
    start digit-length
    ( #start )
    \ S" Starting initial condition detection" type .S cr
    dup n >= if
        \ start has at least n digits
        ( #start )
        dup n mod
        ( #start #start%n )
        dup 0= if
            drop
            ( #start )
            \ start could be made of n repeats
            \ go with the first 1/n of the number, or maybe +1
            dup 
            n /
            ( #start #start/n )
            -
            ( #start-#start/n )
            start swap remove-digits
            ( st )
            dup n
            ( st st n)
            repeat-n-times
            ( st ststst )
            start <
            ( st ststst<start )
            if
                \ S" Prefix was too small, going bigger" type .S cr
                1+
            endif
            ( st+?1 )
        else
            \ start cannot be made of n repeats, 
            \ and n - #start mod n is how wrong it is
            \ i.e. 2 duplicates and start is 3 digits
            \ 2 - 3%2 = 1
            \ 3 + 1 = 4 is how many digits we need to go to
            \ = syntax highlighting!!!
            ( #start #start%n )
            n swap -
            ( #start n-#start%n )
            +
            ( #digits )
            
            dup 1- pow10
            ( #digits newstart )
            stop > if
                \ S" Early exit, went too big" type .S cr
                drop
                0
                exit
            endif
            n /
            ( #digits/n )
            1- pow10
            ( prefix )
        endif
    else
        \ start is too small, we'll start with 1 repeated n times
        \ which is guaranteed to be bigger than start
        drop
        1
    endif
    ( prefix )
    0 >r
    \ S" Starting search with prefix" type .S cr
    begin
        dup n repeat-n-times
        dup stop <=
        ( prefix repeated in-range? )
        \ S" Checking for match?" type .S cr
    while
        \ dup 0 <= if start stop n .S assert( 0 ) endif
        \ S" found a match" type .S cr
        r> + >r
        1+
    repeat
    2drop
    r>
;

: invalid-in-range-2 { start stop -- invalid-sum }
    \ We can look for doubles, triples, quads all the way up to Nx repeats
    \ We need to be aware of doubling up though: 1111 can be a double and a quad
    \ although, anything which can be a quad can also be a double
    \ so really we just need to look at prime repeats
    \ 111111 can be 111|111 or 11|11|11 or 1|1|1|1|1|1 though, so even looking at
    \ if abcdef = abcabc and ababab then c=a, b=a, it is all the same
    \ just prime can still have doubles
    \ but, if we add primes and subtract composite which are not prime powers
    \ 2 3 5 -6 7 -10
    \ because 4, 8 are covered entirely by 2, 9 is covered by 3,
    \ and 6 is both 3 and 2 so we need to remove one count, similarly 10 is 2 and 5

    0 start stop 2 repeats-n-times-sum +
    start stop 3 repeats-n-times-sum +
    start stop 5 repeats-n-times-sum +
    start stop 6 repeats-n-times-sum -
    start stop 7 repeats-n-times-sum +
    start stop 10 repeats-n-times-sum -
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

: day-2-part-2 ( fd -- result )
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
    invalid-in-range-2 +
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

: test-day-2-part-2
  s" day2.example" r/o open-file throw ( -- fd )
  >r
  r@ day-2-part-2  . \ assert( 4174379265 = )
  r>
  close-file throw
;

( Expect to get 70187097315 )
: do-day-2-part-2
  s" day2.in" r/o open-file throw ( -- fd )
  >r
  r@ day-2-part-2 .
  r>
  close-file throw
;