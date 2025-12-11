require ../common/common.fs

CREATE day-11-buf 100 allot

struct
    cell% field list-length
    cell% 32 * field list-data
end-struct list32%

: make-list ( -- list )
    list32% %alloc
    dup list32% %size erase
;

: list32-append { list v -- }
    list list-length @ 
    assert( dup 32 < )
    cells
    list list-data + v swap !
    1 list list-length +!
;

: chars-to-int ( ptr -- n )
    dup c@ 'a' - 5 lshift
    over 1+ c@ 'a' - + 5 lshift
    swap 2 + c@ 'a' - +
    1+
;

: parse-input ( fs -- map )
    >r
    1024 create-associative-map
    { map }

    begin
        day-11-buf 100 r@ read-line throw
        ( #read eof? )
    while
        day-11-buf + -1 swap c!

        day-11-buf
        dup chars-to-int
        { from }
        make-list { from-list }
        map from from-list associative-map-set drop
        4 +
        begin
            ( ptr )
            1+
            dup chars-to-int
            ( ptr to )
  
            from-list swap list32-append
            3 +
            dup c@ 32 <>
        until
        drop
    repeat
    drop
    r> drop
    map
;

: propagate-rec { map reachability-map node -- paths }
    \ ." Propagating" .S cr
    reachability-map node associative-map-lookup
    if
        exit
    endif
    0
    map node associative-map-lookup
    if
        ( acc successor-list )
        dup list-data swap 
        list-length @ 0 do
            ( acc successor-ptr )
            map reachability-map 2 pick @ recurse
            ( acc successor-ptr vnext )
            rot + swap cell+    
        LOOP
        drop
    endif
    reachability-map node 2 pick associative-map-set drop
;

: propagate { map start destination -- answer }
    1024 create-associative-map { reachability-map }
    reachability-map destination 1 associative-map-set drop
    \ invariant: if we set something in the reachability map then it is
    \            fixed, there are that many ways from that node to the destination
    map reachability-map start propagate-rec
    reachability-map free-associative-map
;

: propagate-via-rec { map reach-both reach-dac reach-fft reach-none state node -- paths }
    \ ." recursing" state node .S 2drop cr
    reach-both reach-dac reach-fft reach-none state pick { reach-relevant } drop drop drop drop
    reach-relevant node associative-map-lookup
    if
        exit
    endif

    [ S" dac" drop chars-to-int ] literal node = if
        state 2 or
    else
        [ S" fft" drop chars-to-int ] literal node = if
            state 1 or
        else
            state
        endif
    endif
    { nextstate }

    0
    map node associative-map-lookup if
        ( acc successor-list )
        dup list-data swap 
        list-length @ 0 do
            ( acc successor-ptr )
            map reach-both reach-dac reach-fft reach-none nextstate 6 pick @ recurse
            ( acc successor-ptr vnext )
            rot + swap cell+    
        LOOP
        drop
    else
        \ ." Nowhere more to go" .S cr
    endif
    reach-relevant node 2 pick associative-map-set drop
    \ ." End recursion" .S cr
;

: propagate-via-specified { map start destination -- answer }
    1024 create-associative-map { reach-both }
    1024 create-associative-map { reach-dac }
    1024 create-associative-map { reach-fft }
    1024 create-associative-map { reach-none }
    reach-both destination 1 associative-map-set drop
    map reach-both reach-dac reach-fft reach-none 0 start propagate-via-rec
    reach-both reach-dac reach-fft reach-none
    free-associative-map free-associative-map free-associative-map free-associative-map
;

: day-11-part-1 ( fs -- answer)
    parse-input
    S" you" drop chars-to-int
    S" out" drop chars-to-int 
    propagate
;

: day-11-part-2 ( fs -- answer)
    parse-input
    S" svr" drop chars-to-int
    S" out" drop chars-to-int
    propagate-via-specified
;

: test-day-11-part-1
  s" day11.example" r/o open-file throw ( -- fd )
  >r
  r@ day-11-part-1 assert( 5 = )
  r>
  close-file throw
;

( Expect to get 733 )
: do-day-11-part-1
  s" day11.in" r/o open-file throw ( -- fd )
  >r
  r@ day-11-part-1 .
  r>
  close-file throw
;

: test-day-11-part-2
  s" day11.example2" r/o open-file throw ( -- fd )
  >r
  r@ day-11-part-2 . \ assert( 0 = )
  r>
  close-file throw
;

( Expect to get 290219757077250 )
: do-day-11-part-2
  s" day11.in" r/o open-file throw ( -- fd )
  >r
  r@ day-11-part-2 .
  r>
  close-file throw
;