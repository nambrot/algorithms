module Huffman.Tree exposing (..)

import Dict exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Huffman.CharacterDistribution exposing (..)


type Tree
    = TreeBranch Int Tree Tree
    | TreeLeaf Int Char


toTree : Huffman.CharacterDistribution.Distribution -> Tree
toTree =
    Dict.toList >> List.map (uncurry <| flip TreeLeaf) >> sort >> toTreeH


safeDecode : String -> Tree -> String
safeDecode string tree =
    case decode string tree of
        Ok res ->
            res

        Err res ->
            res


decode : String -> Tree -> Result String String
decode string tree =
    decodeH string tree tree


decodeH : String -> Tree -> Tree -> Result String String
decodeH string root tree =
    case ( tree, String.uncons string ) of
        ( TreeBranch _ left _, Just ( '0', tail ) ) ->
            decodeH tail root left

        ( TreeBranch _ _ right, Just ( '1', tail ) ) ->
            decodeH tail root right

        ( TreeLeaf _ leafChar, Nothing ) ->
            Ok <| String.fromChar <| leafChar

        ( TreeLeaf _ leafChar, Just _ ) ->
            Result.map (String.cons leafChar) (decodeH string root root)

        ( _, _ ) ->
            Err "Parsing went wrong"


sort : List Tree -> List Tree
sort =
    List.sortBy probability


probability : Tree -> Int
probability tree =
    case tree of
        TreeBranch a _ _ ->
            a

        TreeLeaf a _ ->
            a


toTreeH : List Tree -> Tree
toTreeH trees =
    case trees of
        [] ->
            TreeLeaf 1 'a'

        a :: [] ->
            a

        a :: b :: tail ->
            let
                ap =
                    probability a

                bp =
                    probability b

                prob =
                    ap + bp

                branch =
                    TreeBranch prob a b

                newList =
                    branch :: tail
            in
            newList |> sort |> toTreeH


childStyle : Attribute msg
childStyle =
    style
        [ ( "width", "50%" )
        , ( "display", "inline-block" )
        , ( "vertical-align", "top" )
        , ( "text-align", "center" )
        ]


parentStyle : Attribute msg
parentStyle =
    style
        [ ( "width", "100%" )
        , ( "padding-top", "50px" )
        , ( "position", "relative" )
        ]


leafStyle : Attribute msg
leafStyle =
    style
        [ ( "border-radius", "40px" )
        , ( "border", "2px solid black" )
        , ( "padding", "10px" )
        , ( "font-size", "24px" )
        ]


branchStyle : Attribute msg
branchStyle =
    style
        [ ( "position", "absolute" )
        , ( "top", "0px" )
        , ( "left", "calc(45%)" )
        ]


toHtml : Tree -> Html msg
toHtml tree =
    case tree of
        TreeLeaf prob char ->
            span
                [ leafStyle ]
                [ char |> toString |> text ]

        TreeBranch prob left right ->
            div
                [ parentStyle
                ]
                [ div
                    [ childStyle
                    ]
                    [ toHtml left ]
                , div
                    [ childStyle
                    ]
                    [ toHtml right ]
                , div
                    [ branchStyle ]
                    [ text "/(x)\\" ]
                ]
