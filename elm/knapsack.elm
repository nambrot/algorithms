module Main exposing (..)

import Array exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


type alias Entry =
    { value : Int
    , weight : Int
    }


type alias Model =
    { entries : List Entry
    , capacity : Int
    }


init : ( Model, Cmd Msg )
init =
    ( { entries =
            [ { value = 10, weight = 3 }
            , { value = 12, weight = 4 }
            , { value = 25, weight = 9 }
            ]
      , capacity = 10
      }
    , Cmd.none
    )


type Msg
    = UpdateEntries String
    | UpdateCapacity String


view : Model -> Html Msg
view model =
    div []
        [ h1 []
            [ text "Knapsack Problem" ]
        , hr [] []
        , p []
            [ text "" ]
        , h3 []
            [ text "Input Entries" ]
        , textarea
            [ style [ ( "height", "250px" ) ]
            , onInput UpdateEntries
            ]
            []
        , h3 []
            [ text "Input Capacity" ]
        , input
            [ type_ "text"
            , value (toString model.capacity)
            , onInput UpdateCapacity
            ]
            []
        , h3 []
            [ text ("Total value is: " ++ (cacheFrom model |> Array.toList |> List.reverse |> List.head |> Maybe.withDefault Array.empty |> Array.toList |> List.reverse |> List.head |> Maybe.withDefault 0 |> toString)) ]
        , h3 []
            [ text "Selected Items" ]
        , ul []
            (knapsack model |> List.map (\entry -> li [] [ text ("Value: " ++ toString entry.value ++ ", Weight: " ++ toString entry.weight) ]))
        ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateCapacity string ->
            ( { model | capacity = string |> String.toInt |> Result.withDefault 0 }, Cmd.none )

        UpdateEntries string ->
            ( { model | entries = string |> String.lines |> List.filterMap parseEntry }, Cmd.none )


parseEntry : String -> Maybe Entry
parseEntry =
    String.words >> List.map String.toInt >> toEntry


toEntry : List (Result String Int) -> Maybe Entry
toEntry list =
    case list of
        (Ok value) :: (Ok weight) :: [] ->
            Just { value = value, weight = weight }

        _ ->
            Nothing


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


type alias Cache =
    Array (Array Int)


cacheFrom : Model -> Cache
cacheFrom model =
    initCache model.entries model.capacity |> populateRestOfCache model.entries 0 0 model.capacity


knapsack : Model -> List Entry
knapsack model =
    cacheFrom model |> reconstruct model


initCache : List Entry -> Int -> Cache
initCache entries capacity =
    let
        emptyRowFactory =
            \n -> Array.initialize (capacity + 1) <| \n -> 0
    in
    Array.initialize (List.length entries) emptyRowFactory


populateRestOfCache : List Entry -> Int -> Int -> Int -> Cache -> Cache
populateRestOfCache currentEntry i j capacity cache =
    case ( capacity < j, currentEntry ) of
        ( _, [] ) ->
            cache

        ( True, head :: tail ) ->
            -- checked all capacities for the current item
            populateRestOfCache tail (i + 1) 0 capacity cache

        ( False, head :: tail ) ->
            nextValue i j head cache
                |> updateCache i j cache
                |> populateRestOfCache currentEntry i (j + 1) capacity


reconstruct : Model -> Cache -> List Entry
reconstruct model cache =
    reconstructH (List.reverse model.entries) cache (List.length model.entries - 1) model.capacity |> List.reverse


reconstructH : List Entry -> Cache -> Int -> Int -> List Entry
reconstructH list cache i j =
    case ( list, lookup i j cache ) of
        ( [], _ ) ->
            []

        ( ({ weight } as head) :: tail, targetValue ) ->
            if didIPickElement i j head cache then
                head :: reconstructH tail cache (i - 1) (j - weight)
            else
                reconstructH tail cache (i - 1) j


updateCache : Int -> Int -> Cache -> Int -> Cache
updateCache i j cache val =
    let
        updatedRow =
            Array.get i cache
                |> Maybe.withDefault Array.empty
                |> Array.set j val
    in
    Array.set i updatedRow cache


lookup : Int -> Int -> Cache -> Int
lookup i j cache =
    Array.get i cache |> Maybe.withDefault Array.empty |> Array.get j |> Maybe.withDefault 0


nextValue : Int -> Int -> Entry -> Cache -> Int
nextValue i j entry cache =
    compareInCache i j entry cache |> Tuple.first


didIPickElement : Int -> Int -> Entry -> Cache -> Bool
didIPickElement i j entry cache =
    compareInCache i j entry cache |> Tuple.second


compareInCache : Int -> Int -> Entry -> Cache -> ( Int, Bool )
compareInCache i j { value, weight } cache =
    let
        doesItFit =
            weight <= j

        prior =
            lookup (i - 1) j cache

        possible =
            lookup (i - 1) (j - weight) cache + value
    in
    if doesItFit && possible > prior then
        ( possible, True )
    else
        ( prior, False )



-- cacheToHtml : Int -> Cache -> Html msg
-- cacheToHtml capacity cache =
--     table []
--         [ thead []
--             [ tr []
--                 (List.range 0 capacity |> List.map numberToTd)
--             ]
--         , tbody []
--             (Array.toList <|
--                 Array.map (tr [] << Array.toList << Array.map numberToTd) cache
--             )
--         ]
-- tdStyle : Attribute msg
-- tdStyle =
--     style
--         [ ( "padding", "0px 16px" )
--         , ( "text-align", "center" )
--         ]
-- numberToTd : Int -> Html msg
-- numberToTd =
--     toString >> text >> List.singleton >> td [ tdStyle ]
