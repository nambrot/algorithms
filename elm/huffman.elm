module Main exposing (..)

import Dict exposing (..)
import Html exposing (..)
import Html.Events exposing (onInput)
import Huffman.CharacterDistribution exposing (Distribution)
import Huffman.Tree exposing (..)
import Huffman.LookupTable exposing (..)

-- import Result exposing (Maybe, Nothing)


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    { tree : Huffman.Tree.Tree
    , distribution : Distribution
    , lookupTable : Huffman.LookupTable.LookupTable
    , encodedString : String
    , decodedString : String
    }


init : ( Model, Cmd Msg )
init =
    ( { tree = Huffman.Tree.TreeLeaf 1 ' '
      , distribution = Dict.empty
      , lookupTable = Dict.empty
      , encodedString = ""
      , decodedString = ""
      }
    , Cmd.none
    )


type Msg
    = UpdateSampleText String
    | UpdateStringToEncode String
    | UpdateStringToDecode String


view : Model -> Html Msg
view model =
    div []
        [ h1 []
            [ text "Huffman encoding" ]
        , hr [] []
        , h3 []
            [ text "Sample Text"]
        , textarea [ onInput UpdateSampleText ] []
        , h3 []
            [ text "Character Distribtion" ]
        , Huffman.CharacterDistribution.toHtml model.distribution
        , h3 []
            [ text "Tree" ]
        , Huffman.Tree.toHtml model.tree
        , h3 []
            [ text "Lookup Table" ]
        , Huffman.LookupTable.toHtml model.lookupTable
        , h3 []
            [ text "String to encode"]
        , textarea [ onInput UpdateStringToEncode ] []
        , p []
          [ text <| "The encoded string is: " ++ model.encodedString]
        , h3 []
            [ text "String to decode"]
        , textarea [ onInput UpdateStringToDecode ] []
        , p []
          [ text <| "The decoded string is: " ++ model.decodedString]
        ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateSampleText a ->
            let
                distribution =
                    Huffman.CharacterDistribution.toDistribution a

                tree =
                    Huffman.Tree.toTree distribution

                table = Huffman.LookupTable.toLookupTable tree
            in
            ( { model | tree = tree, distribution = distribution, lookupTable = table }, Cmd.none )
        UpdateStringToEncode string ->
          ( { model | encodedString = Huffman.LookupTable.encode string model.lookupTable}, Cmd.none)
        UpdateStringToDecode string ->
          ( { model | decodedString = Huffman.Tree.safeDecode string model.tree }, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
