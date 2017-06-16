module Main exposing (..)

import Array exposing (Array)
import Html exposing (..)
import Html.Events exposing (onInput)


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Model =
    List Int


init : ( Model, Cmd Msg )
init =
    ( [], Cmd.none )


type Msg
    = UpdateInput String


view : Model -> Html Msg
view model =
    div []
        [ h1 []
            [ text "Maximum Weight Independent Set" ]
        , hr [] []
        , h3 []
            [ text "Input" ]
        , textarea [ onInput UpdateInput ] []
        , h3 []
            [ text "The populated array is:" ]
        , p []
            [ text <| printList <| Array.toList <| getArr <| model ]
        , h3 []
            [ text "The max weight independent set is:" ]
        , p []
            [ text <| printList <| maxWeightIndependentSet <| model ]
        ]

printList : List Int -> String
printList list =
  "["
    ++ (String.join ", " <| List.map toString <| list)
    ++ "]"

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateInput string ->
            ( string |> String.lines |> List.map (String.toInt >> Result.withDefault 0), Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


maxWeightIndependentSet : List Int -> List Int
maxWeightIndependentSet list =
    case list of
        [] ->
            []

        [ a ] ->
            [ a ]

        head :: tail ->
            forward head tail
                |> reconstruct (List.reverse list) (List.length list - 1)
                |> List.reverse


getArr : List Int -> Array Int
getArr list =
    case list of
        [] ->
            Array.empty

        [ a ] ->
            Array.empty

        head :: tail ->
            forward head tail


forward : Int -> List Int -> Array Int
forward head tail =
    build tail 1 <|
        Array.set 0 head <|
            Array.initialize (List.length (head :: tail)) <|
                \n -> 0


build : List Int -> Int -> Array Int -> Array Int
build list i arr =
    case ( Array.get (i - 1) arr, Array.get (i - 2) arr, list ) of
        ( _, _, [] ) ->
            arr

        ( Just a, Nothing, b :: tail ) ->
            build tail (i + 1) <| Array.set i (max a b) arr

        ( Just a, Just b, c :: tail ) ->
            build tail (i + 1) <| Array.set i (max (b + c) a) arr

        ( _, _, _ ) ->
            arr



-- should not really get here
-- list is reverted since its singly linked


reconstruct : List Int -> Int -> Array Int -> List Int
reconstruct list i arr =
    case ( list, i, Array.get i arr, Array.get (i - 1) arr, Array.get (i - 2) arr ) of
        ( [ lastEl ], 0, Just a, Nothing, Nothing ) ->
            -- end of iteration
            [ lastEl ]

        ( el1 :: el2 :: [], _, _, Just _, Nothing ) ->
            -- near the end
            [ max el1 el2 ]

        ( el1 :: el2 :: tail, _, Just a, Just _, Just c ) ->
            -- main iteration
            if a == c + el1 then
                -- We are using el1, skip neighbor
                el1 :: reconstruct tail (i - 2) arr
            else
                -- el1 is not in the set
                reconstruct (el2 :: tail) (i - 1) arr

        _ ->
            -- should not get here
            [ 0 ]
