module Huffman.LookupTable exposing (..)

import Dict exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Huffman.Tree exposing (Tree(..))


type alias LookupTable =
    Dict Char String

encode : String -> LookupTable -> String
encode string table =
  let
      foldF char acc = Dict.get char table |> Maybe.withDefault "NOTENCODED" |> String.append acc
  in
      string |> String.toList |> List.foldl foldF ""
toLookupTable : Tree -> LookupTable
toLookupTable tree =
    toLookupTableH tree "" Dict.empty


toLookupTableH : Tree -> String -> LookupTable -> LookupTable
toLookupTableH tree prefix table =
    case tree of
        TreeBranch _ left right ->
            table |> toLookupTableH left (prefix ++ "0") |> toLookupTableH right (prefix ++ "1")

        TreeLeaf _ char ->
            Dict.insert char prefix table


toHtml : LookupTable -> Html msg
toHtml = toList >> List.map pairToHtml >> div []


pairToHtml : ( Char, String ) -> Html msg
pairToHtml (char, string) = div [] [ text (toString char ++ ": " ++ string) ]
