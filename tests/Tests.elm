module Tests exposing (..)

import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Test exposing (..)

suite : Test
suite =
    describe "The main module"
        [ describe "default tests"
            [ test "default test" <|
                \_ ->
                    let
                        palindrome =
                            "hannah"
                    in
                        Expect.equal 10 23
            ]
        ]
