module Huffman.CharacterDistribution exposing (..)

import Dict exposing (..)
import Html exposing (..)


type alias Distribution =
    Dict Char Int


toDistribution : String -> Distribution
toDistribution =
    let
        add1 =
            Just << (+) 1 << Maybe.withDefault 0

        foldF =
            flip Dict.update add1

        count =
            List.foldl foldF Dict.empty
    in
    String.toList >> count


toHtml : Distribution -> Html msg
toHtml distribution =
    let
        list =
            Dict.toList distribution

        mapF tuple =
            let
                ( char, prob ) =
                    tuple
            in
            p []
                [ text ("Character '" ++ toString char ++ "' has a prob of " ++ toString prob)
                ]
    in
    div [] (List.map mapF list)
